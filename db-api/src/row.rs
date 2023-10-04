use ion_rs;
use ion_rs::IonWriter;
use ion_rs::element::reader::ElementReader;
use ion_rs::IonReader;
use ion_rs::element::writer::TextKind;

use crate::Encoder;
use crate::Decoder;
use crate::envelope::Envelope;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RowDTO {
    table_name: Option<String>,
    values: Vec<Envelope>,
}