use std::collections::HashMap;
use std::rc::Rc;
use crate::types::CellValue;

pub trait DbConfig {
    #[allow(clippy::type_complexity)]
    fn get_supported_types() -> HashMap<String, fn(String) -> Rc<dyn CellValue>>;
}