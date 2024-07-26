use std::collections::HashMap;

use parser::DbBase;
use serde_json::Value;

mod parser;

mod prelude {
    pub const EXTENSION: &str = "rdb";
}

fn main() {
    let mut db = DbBase::from_name("testdb".to_string()).unwrap();
    println!("{:?}", db);
    // db.add_table("cargo".to_string()).unwrap();
    let mut table = db.get_table("cargo".to_string()).unwrap();
    println!("table: {:?}", table);
    table
        .update("some".to_string(), Value::Array(Vec::from([Value::String("one".to_string()), Value::String("value".to_string())]))).unwrap();
        // .get_value("another_key".to_string()).unwrap();
        // .add("some".to_string(), Value::String("new key".to_string())).unwrap()
        // .add("another_key".to_string(), Value::Null).unwrap();
    // println!("value: {:?}", value);
    println!("table: {:?}", table);
}
