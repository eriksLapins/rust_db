use std::{collections::HashMap, fs::{self, File}, io::Write};
use serde_json::Value;

use crate::prelude::*;

pub type DbEntry = HashMap<String, Value>;

#[derive(Debug)]
pub struct DbData {
    path: String,
    data: DbEntry,
    keys: Vec<String>,
}

impl DbData {
    pub fn new(name: &str, data: HashMap<String, Value>) -> Result<Self, String> {
        let file_path = format!("./{name}.{EXTENSION}");
        let file = File::create_new(&file_path).unwrap();
        serde_json::to_writer(file, &data).unwrap();
        parse(file_path.as_str())
    }
    pub fn getValue(&self, key: &str) -> Result<Value, String> {
        if self.keys.contains(&key.to_string()) {
            return Ok(self.data.clone().get(key).unwrap().to_owned())
        }
        Err("No such key in DB".to_string())
    }
}

pub fn parse(filepath: &str) -> Result<DbData, String> {
    let file_extension = filepath.ends_with(EXTENSION);
    if file_extension == false {
        return Err("Wrong extension".to_string())
    }
    let file = fs::read_to_string(filepath).expect("Unable to get the file");
    let contents: DbEntry = serde_json::from_str(file.as_str()).expect("Unable to serialize");
    let keys: Vec<String> = contents.clone().into_keys().collect();

    Ok(
        DbData {
            path: filepath.to_string(),
            data: contents,
            keys
        }
    )
}