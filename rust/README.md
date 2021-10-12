## rust binding for ejdb2

[EJDB2](https://ejdb.org/) is a single-file embedded document database in c. This project provides a rust binding for it (version 2.61).

## Installation

```toml
[dependencies]
ejdb2-sys = "2.61.0"
ejdb2 = "0.0.5"
serde_json = "1.0"
```

## Example

```rust
extern crate serde_json;
extern crate ejdb2;
extern crate ejdb2_sys;


use ejdb2::ejdbquery::{EJDBQuery, SetPlaceholder};
use ejdb2::ejdb::EJDB;
use serde_json::json;

fn main() {
    EJDB::init().unwrap();
    let mut db = EJDB::new();

    db.open("test.db").unwrap();

    let data = json!({
        "serde_key" : 32,
        "test_val" : [1,2,3],
        "nested" : { "test" : "str"}
    });

    let id = db.put_new("test", &data).unwrap();

    let meta:serde_json::Value = db.info().unwrap();

    println!("db meta {}", meta);

    let result:String = db.get("test", id).unwrap();

    println!("get {}, {}",1, result);

    let mut query: EJDBQuery = EJDBQuery::new("test", "/* | limit :limit skip :skip ");
    query.init().unwrap();

    query.set_placeholder("limit", 0, 3 as i64).unwrap();
    query.set_placeholder("skip", 0, 3 as i64).unwrap();

    let mut result = Vec::<(i64, serde_json::Value)>::new();

    db.exec::<serde_json::Value>(&query, &mut result).unwrap();
    println!("after exec {}", result.len());

    for (id, r) in result {
        println!("id {}, value {}", id, r);
    }
}

```

Internally EJDB uses JBL as a binary format for json. This rust binding supports serde_json <-> JBL conversion. If serde_json is not the json library you use, you could provide your own format convertor.

Implement:

```rust
pub trait EJDBSerializable<T> {
    fn from_jbl(jbl: ejdb2_sys::JBL) -> Result<T, ejdb2_sys::iwrc> ;

    fn to_jbl(&self) -> Result<ejdb2_sys::JBL, ejdb2_sys::iwrc>;
}
```

This binding also supports convertion between a string and a JBL. You can also serialize your format into a json string and convert it to JBL. But this will be slower. 