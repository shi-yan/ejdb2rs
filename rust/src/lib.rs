
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
        
        let mut result = Vec::<(i64, serde_json::Value)>::new();

        db.exec::<serde_json::Value>(&query, &mut result).unwrap();
        println!("after exec {}", result.len());
    
        for (id, r) in result {
            println!("id {}, value {}", id, r);
        }

        Ok(())
    }
}
