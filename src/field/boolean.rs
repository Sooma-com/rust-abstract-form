use crate::Validation;

#[derive(Default, Clone, Debug)]
pub struct Boolean {
    pub tag: String,
    pub label: String,
    pub value: bool,
    pub validations: Vec<std::sync::Arc<Box<dyn Validation>>>,
}
impl Boolean {
    pub fn new(tag: String, label: String, value: bool) -> Self {
        Self {
            tag,
            label,
            value,
            validations: vec![],
        }
    }
    pub fn get_validations(&self) -> &Vec<std::sync::Arc<Box<dyn Validation>>> {
        &self.validations
    }
    pub fn get_validations_mut(&mut self) -> &mut Vec<std::sync::Arc<Box<dyn Validation>>> {
        &mut self.validations
    }
}
