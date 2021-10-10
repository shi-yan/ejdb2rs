## rust binding for ejdb2

[EJDB2](https://ejdb.org/) is a single-file embedded document database in c. This project provides a rust binding for it (version 2.61).

## Installation

```toml
[dependencies]
ejdb2 = "0.0.1"
```

## Example

```rust
extern crate serde_json;

mod ejdb;
mod ejdbquery;
use ejdbquery::{SetPlaceholder, EJDBSerializable};
use serde_json::json;

fn main() {
    ejdb::EJDB::init().unwrap();
    let mut db = ejdb::EJDB::new();

    db.open(&String::from("test.db")).unwrap();

    let data = json!({
        "serde_key" : 32,
        "test_val" : [1,2,3],
        "nested" : { "test" : "str"}
    });

    let id = db.put_new(&String::from("test").as_str(), &data).unwrap();

    let meta:serde_json::Value = db.info().unwrap();

    println!("db meta {}", meta);

    let result:String = db.get(&String::from("test"), id).unwrap();

    println!("get {}, {}",1, result);

    let mut query: ejdbquery::EJDBQuery = ejdbquery::EJDBQuery::new("test", "/* | limit :limit skip :skip ");
    query.init().unwrap();

    query.set_placeholder("limit", 0, 3 as i64).unwrap();
    query.set_placeholder("skip", 0, 3 as i64).unwrap();

    db.exec(&query, |id: i64, doc: String| -> ejdb2_sys::iwrc{
        println!("in callback {} {}",id, doc);
        0
    }).unwrap();

   // db.close();
}

```