mod ejdb;
mod ejdbquery;
use ejdbquery::SetPlaceholder;

fn main() {
    ejdb::EJDB::init();
    let mut db = ejdb::EJDB::new();

    db.open(&String::from("test.db"));

    db.put_new(&String::from("test"),&String::from("{\"test\":32}")).unwrap();

    let meta = db.info().unwrap();

    println!("db meta {}", meta);

    let result = db.get(&String::from("test"), 1).unwrap();

    println!("get {}, {}",1, result);

    let mut query: ejdbquery::EJDBQuery = ejdbquery::EJDBQuery::new("test", "/[test > :age]");
    query.init();

    query.set_placeholder("age", 0, 3 as i64);

    db.exec(&query, |ctx:*mut ejdb_sys::_EJDB_EXEC, doc: ejdb_sys::EJDB_DOC, step: *mut i64| -> ejdb_sys::iwrc{
        println!("in callback");
        0
    });

   // db.close();
}
