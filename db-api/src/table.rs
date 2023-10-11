use ion_rs;
use ion_rs::IonWriter;
use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use crate::Encoder;
use crate::Decoder;
use crate::row::RowDTO;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TableDTO {
    db_name: String,
    name: String,
    rows: Vec<RowDTO>,
}