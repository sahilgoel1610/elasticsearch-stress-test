# Elasticsearch Stress Test Tool

### overview
A tool written in rust inspired from https://github.com/logzio/elasticsearch-stress-test

### Known Bug

You might encounter an error like this if your rate of ingestion is too high. This will kill the script.
```
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 24, kind: Other, message: "Too many open files" }'
```

The above error is becuase of https://github.com/hyperium/hyper/issues/1422. Make sure you run a demon process which would restart the script, if killed, till this issue is address.

## Usage

Clone the source code and do
```
cargo build
cd target/debug
./elasticsearch-stree-test
```
You would need RustLang and cargo installation for this. 

#### List of Parameters

```
Stress test tool for ElasticSearch 1.0
Sahil S. <sahil.s.goel@gmail.com>
Spawn clients and feeds random data to cluster

USAGE:
    elasticsearch-stree-test [OPTIONS] --url <elasticsearch_endpoint>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --bulk-size <bulk-size>                   Number of documents in single index call
    -c, --concurrency <concurrency>
            Number of OS threads that would be forked. Each forked thread would create defined number of Indices.

        --created-indices <created-indices>...    List of indices that are already present. Data would be filled in them
    -d, --documents <documents>                   Number of document that would be created using the templates
    -u, --url <elasticsearch_endpoint>            Elasic search endpoint
    -f, --field-length <field-length>             Number of characters per field
    -i, --indices <indices>                       Number of index each client will created
    -m, --max-fields <max-fields>                 Max number of fields per document
    -r, --replicas <replicas>                     Number of replicas for each index
    -s, --shards <shards>                         Number of shards for each index
    -t, --templates <templates>                   Number of templates that would be generated.
```

Contributions and bugfixes are welcomed. 

Note: perf.sh script will help you to track current rate of ingestion.