# Elasticsearch Stress Test Tool

### overview
A tool written in rust inspired from https://github.com/logzio/elasticsearch-stress-test

### Known Bug

You might encounter an error like this if your rate of ingestion is too high.
```
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 24, kind: Other, message: "Too many open files" }'
```

The above error is becuase of https://github.com/hyperium/hyper/issues/1422. Make sure you run a demon process which would restart the script, if killed, till this issue is address.

## Usage

Clone the source code and do
```
cargo run -paramters
```

#### List of Parameters

```
Stress test tool for ElasticSearch 1.0
Sahil S. <sahil.s.goel@gmail.com>
Spawn clients and feeds random data to cluster

USAGE:
    elasticsearch-stree-test [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --bulk-size <bulk-size>           Number of documents in single index call
        --clients <clients>               Number of parallel client that will be spawned
        --documents <documents>           Number of document templates
    -u, --url <elasticsearch_endpoint>    Sets a custom config file
        --indices <indices>               Number of index each client will created
        --max-fields <max-fields>         Max number of fields per document
        --replicas <replicas>             Number of replicas for each index
        --shards <shards>                 Number of shards for each index
```