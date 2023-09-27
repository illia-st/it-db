use std::rc::Rc;
use crate::types::CellValue;

pub struct Row<T>
where
    T: CellValue + ?Sized
{
    values: Vec<Rc<T>>,
}

impl<T> Row<T>
where
    T: CellValue + ?Sized
{
    pub fn new(values: Vec<Rc<T>>) -> Self {
        Self { values }
    }
    pub fn get_values(&self) -> &[Rc<T>] {
        self.values.as_slice()
    }
}

#[cfg(test)]
mod tests {

}