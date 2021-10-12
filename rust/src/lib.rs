//! This crate is an unofficial rust binding for [EJDB2](https://ejdb.org/). This library is very new, use it at your own risk.
//! 
//! [EJDB2](https://ejdb.org/) is a single-file embedded no-sql database. Some great features of EJDB2 are
//! 1. Being embedded, this makes program distribution easy and removes the need for devops.
//! 2. A database is contained in a single file. Data migration becomes easy, data integrity can be better guaranteed.
//! 3. Fault tolerant storage against catastrophic events, such as a power failure.
//! 
//! There used to be [an old rust binding for EJDB](https://netvl.github.io/ejdb.rs/ejdb/index.html), which is not maintained anymore. 
//! The old binding links EJDB1, however, EJDB had [a major revision since version 2](https://medium.com/@adamansky/ejdb2-41670e80897c). 
//! Therefore I started this crate to expose the latest version (2.61) to rust. The low level binding ejdb2-sys used by this crate is a fork of [the old project](https://netvl.github.io/ejdb.rs/ejdb/index.html).
//! 
//! There are two main structs in this Crate. The first is `struct EJDB` that manipulates a database, for example, inserting new entries or ensuring indices. 
//! The other struct EJDBQuery defines a search query.
//! 
//! A typical workflow includes:
//! 
//! ### Initialize the library:
//! ```rust
//! ejdb::EJDB::init().unwrap();
//! ```
//! 
//! ### Crate a database and open:
//! ``` 
//! let mut db = ejdb::EJDB::new();
//! db.open(&String::from("test.db")).unwrap();
//! ```
//! 
//! Notice that db operations return an error code iwrc. This is the return code of the key-value storage iowow that EJDB is based upon. 
//! 
//! ### Insert a new row:
//! 
//! ```
//! let data = json!({
//! "serde_key" : 32,
//! "test_val" : [1,2,3],
//! "nested" : { "test" : "str"}
//! });
//! let id = db.put_new("test", &data).unwrap();
//! ```
//! 
//! This binding understands two data formats out of box : String or serde_json. You can extend it to add new format support, such as BSON or MessagePack. More on this later.
//! 
//! ### Fetch db meta data:
//! 
//! ```
//! let meta:serde_json::Value = db.info().unwrap();
//! ```
//! 
//! ### Fetch data by primary key:
//! 
//! ```
//! let result:String = db.get("test", id).unwrap();
//! println!("get {}, {}",1, result);
//! ```
//! 
//! ### Search by query with pagination:
//! 
//! ```
//! let mut query: ejdbquery::EJDBQuery = ejdbquery::EJDBQuery::new("test", "/* | limit :limit skip :skip ");
//! query.init().unwrap();
//! query.set_placeholder("limit", 0, 3 as i64).unwrap();
//! query.set_placeholder("skip", 0, 3 as i64).unwrap();
//! let mut result = Vec::<(i64, serde_json::Value)>::new();
//! db.exec::<serde_json::Value>(&query, &mut result).unwrap();
//! for (id, r) in result {
//!    println!("id {}, value {}", id, r);
//! }
//! ```
//! 
//! For details on the query language, please refer to the [original document](https://github.com/Softmotions/ejdb#jql)
//! 
//! The destructor of the object `db` will close the database.
//! 
//! Note, the original implementation provides a built-in webserver for interacting with a database. This function is currently missing from this binding.
//! 
//! To add an adapter for your data format, you need to implement this trait:
//! 
//! ```
//! pub trait EJDBSerializable<T> {
//!   fn from_jbl(jbl: ejdb2_sys::JBL) -> Result<T, ejdb2_sys::iwrc> ;
//!   fn to_jbl(&self) -> Result<ejdb2_sys::JBL, ejdb2_sys::iwrc>;
//! }
//! ```
//! 



extern crate ejdb2_sys;
extern crate serde_json;

pub mod ejdb;
pub mod ejdbquery;

pub use ejdbquery::{SetPlaceholder, EJDBSerializable, EJDBQuery};
pub use ejdb::EJDB;

use serde_json::json;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db() -> Result<(), String> {
        
        ejdb::EJDB::init().unwrap();
        let mut db = ejdb::EJDB::new();
    
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
    
        let mut query: ejdbquery::EJDBQuery = ejdbquery::EJDBQuery::new("test", "/* | limit :limit skip :skip ");
        query.init().unwrap();
    
        query.set_placeholder("limit", 0, 3 as i64).unwrap();
        query.set_placeholder("skip", 0, 3 as i64).unwrap();
        
        let mut result = Vec::<(i64, serde_json::Value)>::new();

        db.exec::<serde_json::Value>(&query, &mut result).unwrap();
        println!("after exec {}", result.len());
    
        for (id, r) in result {
            println!("id {}, value {}", id, r);
        }

        Ok(())
    }
}
