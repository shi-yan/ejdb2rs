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

    //db.del(&String::from("test"), 1).unwrap();

    //let result2:String = db.get(&String::from("test"), 1).unwrap();
    
    //println!("get after del {}, {}",1, result2);

    let mut query: ejdbquery::EJDBQuery = ejdbquery::EJDBQuery::new("test", "/ = :age");
    query.init().unwrap();

    query.set_placeholder("age", 0, 3 as i64).unwrap();

    db.exec(&query, |doc: String| -> ejdb_sys::iwrc{
        println!("in callback {}", doc);
        0
    }).unwrap();

   // db.close();
}
