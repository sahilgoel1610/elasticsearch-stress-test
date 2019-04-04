extern crate futures;
extern crate reqwest;
extern crate tokio;
extern crate tokio_core;
#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate lazy_static;
extern crate rand;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use futures::future::join_all;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest::async::Client;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
// use tokio_core;

#[derive(Debug, Serialize, Deserialize)]
struct PutBody {
    settings: Settings,
}

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    index: Index,
}

#[derive(Debug, Serialize, Deserialize)]
struct Index {
    number_of_shards: i32,
    number_of_replicas: i32,
}

#[derive(Clone, Debug)]
struct Config {
    max_doc_fields: i32,
    max_templates: i32,
    max_documents: i32,
    max_field_length: usize,
    bulk_size: i32,
    number_of_index: i32,
    number_of_shards: i32,
    number_of_replica: i32,
    concurrency: i32,
    created_indices: Vec<String>,
    url: String,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new("Stress test tool for ElasticSearch")
        .version("1.0")
        .author("Sahil S. <sahil.s.goel@gmail.com>")
        .about("Spawn clients and feeds random data to cluster")
        .arg(
            Arg::with_name("elasticsearch_endpoint")
                .short("u")
                .long("url")
                .required(true)
                .help("Elasic search endpoint")
                .takes_value(true),
        ).arg(
            Arg::with_name("indices")
                .long("indices")
                .short("i")
                .help("Number of index each client will created")
                .required(false)
                .takes_value(true),
        ).arg(
            Arg::with_name("documents")
                .long("documents")
                .short("d")
                .help("Number of document that would be created using the templates")
                .required(false)
                .takes_value(true),
        )
        //    .arg(Arg::with_name("clients")
        //         .long("clients")
        //         .
        //        .help("Number of parallel client that will be spawned")
        //        .required(false)
        //        .takes_value(true))
        .arg(
            Arg::with_name("shards")
                .long("shards")
                .short("s")
                .help("Number of shards for each index")
                .required(false)
                .takes_value(true),
        ).arg(
            Arg::with_name("replicas")
                .long("replicas")
                .short("r")
                .help("Number of replicas for each index")
                .required(false)
                .takes_value(true),
        ).arg(
            Arg::with_name("bulk-size")
                .short("bs")
                .long("bulk-size")
                .help("Number of documents in single index call")
                .required(false)
                .takes_value(true),
        ).arg(
            Arg::with_name("max-fields")
                .short("mf")
                .long("max-fields")
                .help("Max number of fields per document")
                .required(false)
                .takes_value(true),
        ).arg(
            Arg::with_name("templates")
                .short("tpl")
                .long("templates")
                .help("Number of templates that would be generated.")
                .required(false)
                .takes_value(true),
        ).arg(
            Arg::with_name("concurrency")
                .short("con")
                .long("concurrency")
                .help("Number of OS threads that would be forked. Each forked thread would create defined number of Indices.")
                .required(false)
                .takes_value(true),
        ).arg(
            Arg::with_name("field-length")
                .short("fl")
                .long("field-length")
                .help("Number of characters per field")
                .required(false)
                .takes_value(true),
        ).arg(
            Arg::with_name("created-indices")
                // .short("c")
                .multiple(true)
                .long("created-indices")
                .help("List of indices that are already present. Data would be filled in them")
                .required(false)
                .takes_value(true),
        ).get_matches();

    let mut children = vec![];
    let mut _empty_index : Vec<_> = matches.values_of("file").unwrap().map(String::from).collect();
    let mut config = Config {
        max_doc_fields: matches
            .value_of("max-fields")
            .unwrap_or("100")
            .parse()
            .unwrap(),
        max_documents: matches
            .value_of("documents")
            .unwrap_or("1000")
            .parse()
            .unwrap(),
        max_templates: matches
            .value_of("templates")
            .unwrap_or("10")
            .parse()
            .unwrap(),
        bulk_size: matches
            .value_of("bulk-size")
            .unwrap_or("100")
            .parse()
            .unwrap(),
        number_of_index: matches.value_of("indices").unwrap_or("5").parse().unwrap(),
        number_of_shards: matches.value_of("shards").unwrap_or("20").parse().unwrap(),
        number_of_replica: matches.value_of("replicas").unwrap_or("1").parse().unwrap(),
        concurrency: matches
            .value_of("concurrecny")
            .unwrap_or("10")
            .parse()
            .unwrap(),
        max_field_length: matches
            .value_of("field-length")
            .unwrap_or("50")
            .parse()
            .unwrap(),
        url: matches
            .value_of("elasticsearch_endpoint")
            .unwrap()
            .to_string(),
        created_indices: vec![],
    };

    println!("Final Config {:?}", config);
    // config.created_indices.push(String::from("ugufvqistiw7ekw"));
    // config.created_indices.push(String::from("dy9jfnvfroumc5x"));
    // config.created_indices.push(String::from("q9vz025taljx2vk"));

    let mut actual_concurrency = config.concurrency;
    if config.created_indices.len() > 0 {
        actual_concurrency = config.created_indices.len() as i32;
    }

    for i in 0..actual_concurrency {
        // Spin up another thread
        // config = config.clone();
        let c_copy = config.clone();
        children.push(thread::spawn(move || {
            // println!("this is thread number {}", i);
            worker(c_copy, i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
    Ok(())
}

fn worker(config: Config, worker_num: i32) -> () {
    let client = Client::builder()
        .timeout(Duration::from_secs(100))
        .build()
        .unwrap();
    let mut created_indicies: Vec<String>;
    if config.created_indices.len() > 0 {
        created_indicies = vec![
            config
                .created_indices
                .into_iter()
                .nth(worker_num as usize)
                .expect(""),
        ];
    } else {
        created_indicies = create_index(config.clone());
    };

    let created_documents = create_documents(
        config.max_documents,
        config.max_templates,
        config.max_doc_fields,
        config.max_field_length,
    );

    // let mut core = tokio_core::reactor::Core::new();
    // let map_error = |e| Error::from(e);
    let bulk_url = config.url + "/_bulk";
    loop {
        let core = tokio_core::reactor::Core::new();
        let mut workers_vec = Vec::new();
        for _ in 0..20 {
            let p_url = reqwest::Url::parse(&bulk_url);
            println!("forking async request");
            let w1 = client
                .post(p_url.unwrap())
                .body(create_bulk_string(
                    &created_indicies,
                    &created_documents,
                    config.bulk_size,
                )).header(CONTENT_TYPE, "application/json")
                .send();
            workers_vec.push(w1);
            // match res {
            //     Ok(mut resp) => {
            //         // println!("API CALL BODY {:?}", resp.text());
            //         println!("API CALL status {}", resp.status());
            //     },
            //     Err(e) => {
            //         println!("Some error while creating bulk request {:?}", e);

            //     }
            // }
        }
        println!("going to join reqeuest");

        let work = join_all(workers_vec);

        println!("joined request");
        // tokio::run(work);
        for response in core.unwrap().run(work).into_iter() {
            println!("{:?}", response);
        }
    }
}

fn create_index(config: Config) -> Vec<String> {
    let mut indices = Vec::new();
    let slash = "/".to_string();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(100))
        .build()
        .unwrap();
    for _ in 0..config.number_of_index {
        let mut ind = generate_random_string(15).to_lowercase();
        let index = Index {
            number_of_replicas: config.number_of_replica,
            number_of_shards: config.number_of_shards,
        };
        let settings = Settings { index: index };

        let new_post = PutBody { settings: settings };

        let mut f_url = "".to_string();
        f_url.push_str(&config.url);
        f_url.push_str(&slash);
        f_url.push_str(&ind);
        println!("final URL {}", f_url);
        let p_url = reqwest::Url::parse(&f_url);
        let mut res = client.put(p_url.unwrap()).json(&new_post).send();

        match res {
            Ok(mut resp) => {
                // println!("API CALL BODY {:?}", resp.text());
                println!("API CALL status {}", resp.status());
                if resp.status().is_success() {
                    println!("URL {}", f_url);
                    println!("Index created: {}", ind);
                    indices.push(ind);
                }
            }
            Err(e) => println!("Some error while creating index {:?}", e),
        }
    }
    return indices;
}

fn create_bulk_string(indices: &Vec<String>, documents: &Vec<String>, bulk_size: i32) -> String {
    let mut bulk_str = "".to_owned();
    let new_line = "\n".to_string();
    let ind = thread_rng().choose(&indices).unwrap();
    let data = json!({"index": {"_index": ind, "_type": "stresstest"}}).to_string();
    for _ in 0..bulk_size {
        bulk_str.push_str(&data);
        bulk_str.push_str(&new_line);
        let mut doc = thread_rng().choose(&documents).unwrap();
        bulk_str.push_str(doc);
        bulk_str.push_str(&new_line);
    }

    // println!("bulk string produced {}", bulk_str);
    return bulk_str;
}

fn create_documents(
    max_documents: i32,
    num_templates: i32,
    num_fields: i32,
    field_length: usize,
) -> Vec<String> {
    let templates = generate_templates(num_templates, num_fields, field_length);
    let mut rng = thread_rng();
    let mut documents = Vec::new();
    for _ in 0..max_documents {
        let templ = rng.choose(&templates).unwrap();
        let doc_string = serde_json::to_string(&fill_template(templ.clone())).unwrap();
        documents.push(doc_string);
    }

    println!("this should take time");
    return documents;
}

fn generate_template(num_fields: i32, field_length: usize) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for _ in 0..num_fields {
        map.insert(
            generate_random_string(5),
            generate_random_string(field_length),
        );
        map.insert(
            generate_random_string(5),
            generate_random_string(field_length),
        );
    }

    return map;
}

fn generate_templates(
    num_templates: i32,
    num_fields: i32,
    field_length: usize,
) -> Vec<HashMap<String, String>> {
    let mut map_vec = Vec::new();
    for _ in 0..num_templates {
        map_vec.push(generate_template(num_fields, field_length))
    }
    return map_vec;
}

fn generate_random_string(max_len: usize) -> String {
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(max_len)
        .collect();
}

fn fill_template(templ: HashMap<String, String>) -> HashMap<String, String> {
    let mut doc = HashMap::new();
    for (k, _v) in &templ {
        doc.insert(k.to_string(), generate_random_string(200));
    }
    return doc;
}
