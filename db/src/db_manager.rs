use std::cell::RefCell;
use std::fs;
use std::fs::{File, Metadata};
use std::io::Read;
use std::iter::Map;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::db::{Database, DatabaseBuilder};
use core::types::CellValue;
use toml::Value;
// Can operate with one db at the time
#[derive(Debug)]
struct DatabaseConfig {
    supported_types: Vec<String>,
}

#[derive(Default)]
pub struct DatabaseManager {
    supported_types: Map<String, fn(String) -> Rc<dyn CellValue>>,
    database: RefCell<Option<Database>>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        let file_path = format!("{}/.config/config.toml", env!("CARGO_MANIFEST_DIR"));

        // Open the file for reading
        let mut file = File::open(file_path).expect("Failed to open file");

        // Read the file's contents into a String
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str)
            .expect("Failed to read file");

        // Parse the TOML string into a toml::Value object
        let parsed_toml: Value = toml::from_str(&toml_str).expect("Failed to parse TOML");
    }
    pub fn create_db(&self, name: String, location: String) -> Result<(), String> {
        if let Ok(metadata) = fs::metadata(location.as_str()) {
            if !metadata.is_dir() {
                return Err("provided path points to the file or symlink".to_string());
            }
        }
        match fs::create_dir(format!("{}/{}", location.as_str(), name.as_str())) {
            Ok(_) => (),
            Err(err) => return Err(format!("couldn't create a directory: {err}"))
        }
        match File::create(format!("{}/{}/{}", location.as_str(), name.as_str(), "tables")) {
            Ok(_) => (),
            Err(err) => return Err(format!("couldn't create a file: {err}"))
        }
        let database = Database::builder()
            .with_location(location)
            .with_name(name)
            .build()
            .unwrap();
        *self.database.borrow_mut().deref_mut() = Some(database);
        Ok(())
    }

    pub fn read_db_from_directory(&self, location: &str) -> Result<(), String> {
        match fs::metadata(location.as_str()) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Err("provided path points to the file or symlink".to_string());
                }
            },
            Err(err) => Err(format!("couldn't open a directory {location}: {err}"))
        };
        // match fs::metadata(format!("{}/{}", location.as_str(), "tables")) {
        //     Ok(metadata) => {
        //         if !metadata.is_dir() {
        //             return Err("provided path points to the file or symlink".to_string());
        //         }
        //     },
        //     Err(err) => Err(format!("couldn't open a table directory: {err}"))
        // };
        // read file location/table using amazon ion
        Ok(())
    }
    pub fn create_table(&self, name: String, ) {
        // 1) check if the table already exists
    }
    pub fn delete_table(&self, table_name: &str) {
        // can be actually called without db being set up
        todo!()
    }
    pub fn add_row(&self, table_name: &str, raw_values: &str) {
        todo!()
    }
    pub fn delete_row(&self, table_name: &str) {
        todo!()
    }
    pub fn close_db(&self) {
        todo!()
    }
    pub fn get_db(&self) {
        todo!()
    }
}

impl Drop for DatabaseManager {
    fn drop(&mut self) {
        todo!()
    }
}