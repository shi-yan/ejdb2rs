extern crate serde_json;

mod ejdb;
mod ejdbquery;
use ejdbquery::{EJDBSerializable, SetPlaceholder};
use serde_json::json;
use std::sync::{Arc, Mutex};

fn main() {
    ejdb::EJDB::init().unwrap();
    let mut db = ejdb::EJDB::new();

    db.open("test.db").unwrap();

    let data = json!({
        "serde_key" : 32,
        "test_val" : [1,2,3],
        "nested" : { "test" : "str"}
    });

    let id = db.put_new("test", &data).unwrap();

    let meta: serde_json::Value = db.info().unwrap();

    println!("db meta {}", meta);

    let result: String = db.get("test", id).unwrap();

    println!("get {}, {}", 1, result);

    let mut query: ejdbquery::EJDBQuery =
        ejdbquery::EJDBQuery::new("test", "/* | limit :limit skip :skip ");
    query.init().unwrap();

    query.set_placeholder("limit", 0, 3 as i64).unwrap();
    query.set_placeholder("skip", 0, 3 as i64).unwrap();

    let mut result = Vec::<(i64, serde_json::Value)>::new();

    db.exec::<serde_json::Value>(&query, &mut result).unwrap();
    println!("after exec {}", result.len());

    for (id, r) in result {
        println!("id {}, value {}", id, r);
    }

    //  db.close();
}
