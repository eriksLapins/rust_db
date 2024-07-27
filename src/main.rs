mod rdb;

mod prelude {
    pub const EXTENSION: &str = "json";
    pub use serde_json::Value;
    pub use crate::rdb::*;
}

use crate::prelude::*;

fn main() {
    let mut db = DbBase::new("testdb".to_string()).unwrap();
    println!("database tables: {}", db);
    db.add_table("testing".to_string()).unwrap();
    println!("database tables: {}", db);
    let table = db.get_table("cargo".to_string()).unwrap();
    println!("table: {}", table);
}
