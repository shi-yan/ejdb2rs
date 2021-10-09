mod ejdb;
mod ejdbquery;
use ejdbquery::{SetPlaceholder, EJDBSerializable};

fn main() {
    ejdb::EJDB::init();
    let mut db = ejdb::EJDB::new();

    db.open(&String::from("test.db"));

    db.put_new(&String::from("test").as_str(),&String::from("{\"test\":32}").as_str()).unwrap();

    let meta:String = db.info().unwrap();

    println!("db meta {}", meta);

    let result:String = db.get(&String::from("test"), 1).unwrap();

    println!("get {}, {}",1, result);

    let mut query: ejdbquery::EJDBQuery = ejdbquery::EJDBQuery::new("test", "/[test > :age]");
    query.init();

    query.set_placeholder("age", 0, 3 as i64);

    db.exec(&query, |doc: String| -> ejdb_sys::iwrc{
        println!("in callback {}", doc);
        0
    });

   // db.close();
}
