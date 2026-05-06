use crate::{Field, FieldSet};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Form {
    pub fieldsets: Vec<FieldSet>,
}
impl Form {
    pub fn new() -> Self {
        Self { fieldsets: vec![] }
    }
    pub fn add_control(&mut self, control: Field) -> &mut Self {
        if self.fieldsets.is_empty() {
            self.fieldsets
                .push(FieldSet::new("default".to_string(), "".to_string()));
        }
        self.fieldsets[0].controls.push(control);
        self
    }
}
