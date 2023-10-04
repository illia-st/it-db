use ion_rs;
use ion_rs::IonWriter;
use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use crate::Encoder;
use crate::Decoder;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TableRow {
    db_name: String,
    table_name: String,
    values: Vec<String>,
}