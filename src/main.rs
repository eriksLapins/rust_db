use std::collections::HashMap;

use serde_json::Value;

mod parser;

mod prelude {
    pub const EXTENSION: &str = "rdb";
}

fn main() {
    let file = parser::parse("./test.rdb");
    match file {
        Ok(contents) => println!("{:?}", contents),
        Err(e) => println!("{:?}", e),
    }

    let new_file = parser::DbData::new("rust", HashMap::from([
        ("New".to_string(), Value::String("Entry".to_string())),
        ("another".to_string(), Value::String("new entry".to_string())),
        ("array".to_string(), Value::Array(vec![
            Value::String("some".to_string()),
            Value::String("new".to_string()),
            Value::String("array".to_string())
        ]))
    ])).unwrap();
    let value = new_file.getValue("another").unwrap();
    println!("{:?}", new_file);
    println!("{:?}", value);
}
