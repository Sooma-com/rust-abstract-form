use crate::Field;
pub mod to_empty_fieldset;
pub use to_empty_fieldset::ToEmptyFieldSet;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct FieldSet {
    pub tag: String,
    pub label: String,
    pub controls: Vec<Field>,
}
impl FieldSet {
    pub fn new(tag: String, label: String) -> Self {
        Self {
            tag,
            label,
            controls: vec![],
        }
    }
    pub fn merge(&mut self, other: &FieldSet) -> &mut Self {
        self.controls.extend(other.controls.clone());
        self
    }
}
