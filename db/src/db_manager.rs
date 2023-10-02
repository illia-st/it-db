use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::format;
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
    supported_types: HashMap<String, fn(String) -> Rc<dyn CellValue>>,
    database: RefCell<Option<Database>>,
}

// impl Default for DatabaseManager {
//     fn default() -> Self {
//         Self {
//             supported_types: Map::<String, fn(String) -> Rc<dyn CellValue>>
//         }
//     }
// }

impl DatabaseManager {
    // creating a database manager
    pub fn new() -> Self {
        // read db manager config
        let file_path = format!("{}/.config/config.toml", env!("CARGO_MANIFEST_DIR"));

        // Open the file for reading
        let mut file = File::open(file_path).expect("Failed to open file");

        // Read the file's contents into a String
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str)
            .expect("Failed to read file");

        // Parse the TOML string into a toml::Value object
        let parsed_toml: Value = toml::from_str(&toml_str).expect("Failed to parse TOML");

        // TODO: debug the method to find out how it will be written
        todo!()
    }
    pub fn create_db(&self, name: &str, location: &str) -> Result<(), String> {
        let _ = self.close_db();
        // check fi such a dir is existing
        if let Ok(metadata) = fs::metadata(location) {
            if !metadata.is_dir() {
                return Err("provided path points to the file or symlink".to_string());
            }
        }
        // create a dir
        match fs::create_dir(format!("{}/{}", location, name)) {
            Ok(_) => (),
            Err(err) => return Err(format!("couldn't create a directory: {err}"))
        }
        // create a dir for tables
        match File::create(format!("{}/{}/{}", location, name, "tables")) {
            Ok(_) => (),
            Err(err) => return Err(format!("couldn't create a file: {err}"))
        }
        // build db using Database::builder()
        let database = Database::builder()
            .with_location(location)
            .with_name(name)
            .build()
            .unwrap();
        *self.database.borrow_mut().deref_mut() = Some(database);
        Ok(())
    }

    pub fn read_db_from_directory(&self, location: &str) -> Result<(), String> {
        // need to close the previous one
        let _ = self.close_db();
        // check if provided location is a dir
        match fs::metadata(location) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Err("provided path points to the file or symlink".to_string());
                }
            },
            Err(err) => return Err(format!("couldn't open a directory {}: {}", location, err))
        };
        // read file location/table using amazon ion
        let tables = match fs::read_dir(format!("{}/{}", location, "tables")) {
            Ok(tables) => tables,
            Err(err) => {
                let err_string = format!("The error is occurred while trying to read tables: {}", err);
                log::error!("{}", err_string.as_str());
                return Err(err_string);
            }
        };
        tables.for_each(|entry| {
            let unwrapped_entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    let err_string = format!("The error is occurred while trying to read tables: {}", err);
                    log::error!("{}", err_string.as_str());
                    return;
                },
            };
            match fs::read(unwrapped_entry.path()) {
                Ok(binary_data) => self.add_table(binary_data),
                Err(err) => log::error!("The error is occurred while trying to read tables: {}", err),
            };
        });
        Ok(())
    }
    fn add_table(&self, raw_table_data: Vec<u8>) {
        todo!("add ion data structures here")
    }
    pub fn create_table(&self, table_name: String) -> Result<(), String> {
        // 1) check if the table already exists
        if self.database.borrow().is_none() {
            let err_string = "There is no active databases in db manager";
            log::error!("{}", err_string);
            return Err(err_string.to_string());
        }
        match File::create(format!("{}/{}/{}", self.database.borrow().as_ref().unwrap().get_location(), "tables", table_name)) {
            Ok(table) => {
                // add ion data type for table adding
            },
            Err(err) => {
                let err_string = format!("Couldn't create a new table {}: {}", table_name, err);
                log::error!("{}", err_string.as_str());
                return Err(err_string);
            },
        }
        Ok(())
    }
    pub fn delete_table(&self, table_name: &str) -> Result<(), String> {
        if self.database.borrow().is_none() {
            let err_string = "There is no active databases in db manager";
            log::error!("{}", err_string);
            return Err(err_string.to_string());
        }
        match fs::remove_file(format!("{}/{}/{}", self.database.borrow().as_ref().unwrap().get_location(), "tables", table_name)) {
            Ok(()) => {
                log::debug!("table {} has been removed", table_name);
                Ok(())
            },
            Err(err) => {
                let err_string = format!("Couldn't delete table {}: {}", table_name, err);
                log::error!("{}", err_string.as_str());
                Err(err_string)
            },
        }
    }
    pub fn add_row(&self, table_name: &str, raw_values: &str) {
        todo!()
    }
    pub fn delete_row(&self, table_name: &str, index: u64) {
        todo!()
    }
    pub fn close_db(&self) -> Result<(), String> {
        if self.database.borrow().is_none() {
            let err_string = "There is no active databases in db manager";
            log::error!("{}", err_string);
            return Err(err_string.to_string());
        }
        let mut db = self.database.borrow_mut();
        db.as_ref().unwrap().get_tables().iter().for_each(|table| {
           // dump all the tables into location/tables/table_name
        });
        *db.deref_mut() = None;
        Ok(())
    }
    pub fn delete_db(&self, location: &str) -> Result<(), String> {
        // TODO: it will be nice to check if the provided location actually is a db
        match fs::remove_dir_all(location) {
            Ok(()) => {
                log::debug!("db in {} has been removed", location);
                Ok(())
            },
            Err(err) => {
                let err_string = format!("Couldn't delete db in {}: {}", location, err);
                log::error!("{}", err_string.as_str());
                Err(err_string)
            },
        }
    }
    pub fn get_db(&self) {
        todo!()
    }
}

impl Drop for DatabaseManager {
    fn drop(&mut self) {
        let _ = self.close_db();
    }
}