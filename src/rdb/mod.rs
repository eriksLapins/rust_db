use std::{collections::HashMap, fmt::Display, fs::{self, File}, path::Path};
use crate::prelude::*;

pub type DbData = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub struct DbBase {
    name: String,
    tables: Vec<String>,
}

impl DbBase {
    pub fn new(name: String) -> Result<Self, String> {
        //! Creates a new DbBase instance if no such exists, otherwise Errors
        let base_path = format!("./{name}");
        let path = Path::new(base_path.as_str());
        if path.exists() && path.is_dir() {
            return Err(format!("Database `{name}` already exists"))
        };
        fs::create_dir(base_path).expect(format!("Unable to create database `{name}`").as_str());

        Ok(Self {
            name,
            tables: Vec::new(),
        })
    }
    pub fn from_name(name: String) -> Result<Self, String> {
        //! Will create a DbBase instance if such a folder exists, otherwise returns an Error
        let base_path = format!("./{name}");
        let path = Path::new(base_path.as_str());
        if path.exists() && path.is_dir() {
            let mut tables = Vec::<String>::new();
            for table in path.read_dir().expect(format!("Unable to read tables in `{name}`").as_str()) {
                if let Ok(table) = table {
                    let table_name = table
                        .file_name().into_string().unwrap()
                        .replace(&format!(".{EXTENSION}"), "");
                    tables.push(table_name);
                }
            }
            return Ok(Self {
                name,
                tables,
            })
        };

        Err(format!("Database `{name}` does not exist"))
    }
    pub fn from_name_or_new(self, name: String) -> Self {
        //! Will create a DbBase instance if such a folder exists, otherwise creates a new one
        let base_path = format!("./{name}");
        let path = Path::new(base_path.as_str());
        if path.exists() && path.is_dir() {
            let mut tables = Vec::<String>::new();
            for table in path.read_dir().expect(format!("Unable to read tables in `{name}`").as_str()) {
                if let Ok(table) = table {
                    let table_name = table
                        .file_name().into_string().unwrap()
                        .replace(&format!(".{EXTENSION}"), "");
                    tables.push(table_name)
                }
            }
            return Self {
                name,
                tables,
            }
        };

        fs::create_dir(base_path).expect(format!("Unable to create database `{name}`").as_str());

        Self {
            name,
            tables: Vec::new(),
        }
    }
    pub fn add_table(&mut self, name: String) -> Result<(&mut Self, DbTable), String> {
        let has_table = self.tables.iter().find(|x| *x == &name);
        if has_table.is_some() {
            return Err(format!("Table `{name}` already exists"))
        } else {
            let table = DbTable::new(name.clone(), &self).unwrap();
            self.tables.push(name);
            Ok((self, table))
        }
    }
    pub fn drop_table(&mut self, name: String) -> Result<&mut Self, String> {
        let index = self.tables.iter().enumerate().find(|&x| x.1 == &name);
        match index {
            Some(n) => {
                let table = self.get_table(name.clone()).unwrap();
                let path = table.get_file_path();
                fs::remove_file(path).expect(format!("Error while removing table `{}` from db `{}`", &name, self.name).as_str());
                
                self.tables.remove(n.0);
                Ok(self)
            }
            None => Err(format!("Table `{name}` does not exist"))
        }
    }
    pub fn get_table(&self, name: String) -> Result<DbTable, String> {
        let has_table = self.tables.iter().find(|x| *x == &name);
        if has_table.is_some() {
            let table = DbTable::get_table(name.clone(), self.name.clone()).unwrap();
            Ok(table)
        } else {
            return Err(format!("Table `{name}` does not exist"))
        }
    }
}

impl Display for DbBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = "".to_string();
        for table in self.tables.iter() {
            output.push_str(format!("\n - {table}").as_str());
        }
        
        write!(f, "{output}")
    }
}

#[derive(Debug, Clone)]
pub struct DbTable {
    name: String,
    parent_name: String,
    data: DbData,
    keys: Vec<String>,
}

impl Display for DbTable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = "\n{".to_string();
        for key in self.keys.clone() {
            output.push_str(format!("\n  {}: {},", &key, self.get(key.clone()).unwrap()).as_str());
        }
        output.push_str("\n}");
        write!(f, "{output}")
        }
}

impl DbTable {
    pub fn new(name: String, base: &DbBase) -> Result<Self, String> {
        let file_path = format!("./{}/{name}.{EXTENSION}", base.name);
        let file = File::create_new(&file_path).unwrap();
        let empty_data: DbData = HashMap::new();
        serde_json::to_writer(file, &empty_data).unwrap();
        Ok(DbTable::get_table(name, base.name.clone()).unwrap())
    }
    pub fn get_table(name: String, base_name: String) -> Result<Self, String> {
        let filepath = format!("./{}/{}.{}", base_name, name, EXTENSION);
        let path = Path::new(&filepath);
        if path.exists() {
            let file = fs::read_to_string(&filepath).expect(format!("Unable to get table `{name}`").as_str());
            let contents: DbData = serde_json::from_str(file.as_str()).expect(format!("Unable to get data from `{name}`").as_str());
            let keys: Vec<String> = contents.clone().into_keys().collect();
    
            Ok(
                Self {
                    name,
                    parent_name: base_name,
                    data: contents,
                    keys
                }
            )
        } else {
            Err(format!("No such table `{name}`"))
        }
    }
    fn get_file_path(&self) -> String {
        format!("./{}/{}.{EXTENSION}", self.parent_name, self.name)
    }
    fn rewrite(&mut self, data: HashMap<String, Value>) -> Result<&mut Self, String> {
        let file_path = self.get_file_path();
        let file = File::options().write(true).open(&file_path).expect(format!("No such file or directory: {file_path}").as_str());
        serde_json::to_writer(file, &data).expect(format!("Failed to write data to `{}` in db `{}`", self.name, self.parent_name).as_str());
        let new_data = DbTable::get_table(self.name.clone(), self.parent_name.clone()).unwrap();
        self.data = new_data.data;
        self.keys = new_data.keys;
        Ok(self)
    }
    pub fn get(&self, key: String) -> Result<Value, String> {
        if self.keys.contains(&key) {
            return Ok(self.data.clone().get(&key).unwrap().to_owned())
        }
        Err(format!("No such key in DB `{}`: {key}", self.name))
    }
    pub fn add(&mut self, key: String, value: Value) -> Result<&mut Self, String> {
        let name = self.name.clone();
        let parent_name = self.parent_name.clone();
        if self.keys.contains(&key) {
            Err(format!("key `{key}` already exists in `{name}` in db `{parent_name}`"))
        } else {
            self.keys.push(key.clone());
            self.data.insert(key, value);
            self.rewrite(self.data.clone()).expect(format!("Error while adding to table `{name}` in db `{parent_name}`").as_str());
            Ok(self)
        }
    }
    pub fn update(&mut self, key: String, value: Value) -> Result<&mut Self, String> {
        let name = self.name.clone();
        let parent_name = self.parent_name.clone();
        if self.keys.contains(&key) {
            self.data.remove(&key);
            self.data.insert(key, value);
            self.rewrite(self.data.clone()).expect(format!("Error while updating table `{name}` in db `{parent_name}`").as_str());
            Ok(self)
        } else {
            Err(format!("key `{key}` does not exist"))
        }
    }
    pub fn remove(&mut self, key: String) -> Result<&mut Self, String> {
        let name = self.name.clone();
        let parent_name = self.parent_name.clone();
        if self.keys.contains(&key) {
            self.data.remove(&key);
            self.rewrite(self.data.clone()).expect(format!("Error while removing table key `{key}` in table `{name}` in db `{parent_name}`").as_str());
            Ok(self)
        } else {
            Err(format!("key `{key}` does not exist"))
        }
    }
}