pub trait Decoder {
    fn decode(data: &[u8]) -> Self;
}
pub trait Encoder {
    fn encode(&self) -> Vec<u8>;
}
pub mod raw_values;
pub mod table_row;
pub mod envelope;