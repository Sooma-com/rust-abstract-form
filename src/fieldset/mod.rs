use crate::Field;
pub mod to_empty_fieldset;
pub use to_empty_fieldset::ToEmptyFieldSet;
pub mod to_field_set;
pub use to_field_set::ToFieldSet;

#[derive(Default, Clone, Debug)]
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
    pub fn field_iter(&self) -> impl Iterator<Item = &Field> {
        self.controls.iter()
    }
    pub fn field_iter_mut(&mut self) -> impl Iterator<Item = &mut Field> {
        self.controls.iter_mut()
    }
}
