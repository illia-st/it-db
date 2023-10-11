use ion_rs;
use ion_rs::element::writer::TextKind;
use ion_rs::IonWriter;
use ion_rs::IonReader;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CharValueDTO {
    command_type: String,
    data: Vec<u8>,
}