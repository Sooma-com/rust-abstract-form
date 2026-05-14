use crate::Field;
use std::sync::Arc;

pub mod to_empty_fieldset;
pub use to_empty_fieldset::ToEmptyFieldSet;
pub mod to_field_set;
pub use to_field_set::ToFieldSet;

#[derive(Default, Clone, Debug)]
pub struct FieldSet {
    pub tag: String,
    pub label: String,
    pub controls: Vec<Arc<Box<dyn Field>>>,
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
    pub fn field_iter(&self) -> impl Iterator<Item = &Arc<Box<dyn Field>>> {
        self.controls.iter()
    }
    pub fn field_iter_mut(&mut self) -> impl Iterator<Item = &mut Arc<Box<dyn Field>>> {
        self.controls.iter_mut()
    }
    pub fn remove_control(&mut self, tag: &str) -> Option<Arc<Box<dyn Field>>> {
        self.controls
            .iter()
            .position(|control| control.get_tag() == tag)
            .map(|index| self.controls.remove(index))
    }
}
