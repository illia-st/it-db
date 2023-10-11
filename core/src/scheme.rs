
#![allow(clippy::type_complexity)]
use std::rc::Rc;
use std::sync::Arc;
use crate::types::CellValue;

pub struct Scheme<T>
where
    T: CellValue + ?Sized,
{
    value_generators: Vec<Arc<fn(String) -> Result<Rc<T>, String>>>,
    // TODO: add columns name
    // columns: Vec<String>,
}
impl<T> Scheme<T>
where
    T: CellValue + ?Sized,
{
    pub fn new(value_generators: Vec<Arc<fn(String) -> Result<Rc<T>, String>>>) -> Self {
        Self { value_generators }
    }
    pub fn builder() -> SchemeBuilder<T> {
        SchemeBuilder::<T>::new()
    }
    pub fn get_validators(&self) -> &[Arc<fn(String) -> Result<Rc<T>, String>>] {
        self.value_generators.as_slice()
    }
}
#[derive(Default)]
pub struct SchemeBuilder<T>
where
    T: CellValue + ?Sized,
{
    value_generators: Vec<Arc<fn(String) -> Result<Rc<T>, String>>>
}

impl<T> SchemeBuilder<T>
where
    T: CellValue + ?Sized,
{
    fn new() -> Self {
        Self { value_generators: Vec::default() }
    }

    pub fn with_type(mut self, generator: Arc<fn(String) -> Result<Rc<T>, String>>) -> Self {
        self.value_generators.push(generator);
        self
    }
    pub fn build(self) -> Scheme<T> {
        Scheme::<T>::new(self.value_generators)
    }
}

#[cfg(test)]
mod tests {
    // TODO: implement tests
}