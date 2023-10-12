use std::collections::HashMap;
use ion_rs;
use ion_rs::IonWriter;
use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use core::db::Database;
use core::table::Table;
use ion_rs::element::writer::TextKind;
use crate::date_value_dto::DateValueDTO;

use crate::Encoder;
use crate::Decoder;
use crate::row_dto::RowDTO;
use crate::table::TableDTO;


#[derive(Debug, PartialEq, Clone)]
pub struct DatabaseDTO {
    name: String,
    location: String,
    tables: Vec<TableDTO>,
}

impl From<DatabaseDTO> for Database {
    fn from(value: DatabaseDTO) -> Self {
        let mut tables = HashMap::with_capacity(value.tables.len());
        value.tables.iter().for_each(|table| {
            tables.insert(table.name.clone(), Table::from(table.clone()));
        });
        let db = Database::new(value.name, value.location);
        db.set_tables(tables);
        db
    }
}

impl DatabaseDTO {
    pub fn new(name: String, location: String, tables: Vec<TableDTO>) -> Self {
        Self {
            name,
            location,
            tables,
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        let buffer: Vec<u8> = Vec::new();

        let binary_writer_builder = ion_rs::BinaryWriterBuilder::new();
        let mut writer = binary_writer_builder.build(buffer.clone()).unwrap();

        writer.step_in(ion_rs::IonType::Struct).expect("Error while creating an ion struct");

        writer.set_field_name("name");
        writer.write_string(&self.name).unwrap();

        writer.set_field_name("location");
        writer.write_string(&self.location).unwrap();

        writer.set_field_name("tables");
        writer.step_in(ion_rs::IonType::List).expect("Error while entering an ion list");
        for table in self.tables.iter() {
            let data = table.encode();
            writer.write_blob(data.as_slice()).unwrap();
        }
        writer.step_out().unwrap();

        writer.step_out().unwrap();
        writer.flush().unwrap();

        writer.output().as_slice().into()
    }
    pub fn decode(data: Vec<u8>) -> Self {
        let mut binary_user_reader = ion_rs::ReaderBuilder::new().build(data).unwrap();
        binary_user_reader.next().unwrap();
        binary_user_reader.step_in().unwrap();

        binary_user_reader.next().unwrap();
        let name = binary_user_reader.read_string().unwrap().to_string();

        binary_user_reader.next().unwrap();
        let location = binary_user_reader.read_string().unwrap().to_string();

        binary_user_reader.next().unwrap();

        let elements = binary_user_reader.read_all_elements().unwrap();
        let mut tables = Vec::<TableDTO>::with_capacity(elements.capacity());
        for element in elements {
            let data = element.as_blob().unwrap();
            tables.push(TableDTO::decode(data.to_vec()));
        }
        binary_user_reader.step_out().unwrap();

        binary_user_reader.step_out().unwrap();

        Self {
            name,
            location,
            tables,
        }
    }
}