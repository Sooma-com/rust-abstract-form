use crate::{Field, FieldSet, errors::Error};

#[derive(Default, Clone, Debug)]
pub struct Form {
    pub fieldsets: Vec<FieldSet>,
}
impl Form {
    pub fn new() -> Self {
        Self { fieldsets: vec![] }
    }
    pub fn from_field_set(fieldset: FieldSet) -> Self {
        Self {
            fieldsets: vec![fieldset],
        }
    }
    pub fn add_control(&mut self, control: Field) -> &mut Self {
        if self.fieldsets.is_empty() {
            self.fieldsets
                .push(FieldSet::new("default".to_string(), "".to_string()));
        }
        self.fieldsets[0].controls.push(control);
        self
    }
    pub fn field_iter(&self) -> impl Iterator<Item = &Field> {
        self.fieldsets
            .iter()
            .flat_map(|fieldset| fieldset.field_iter())
    }
    pub fn field_iter_mut(&mut self) -> impl Iterator<Item = &mut Field> {
        self.fieldsets
            .iter_mut()
            .flat_map(|fieldset| fieldset.field_iter_mut())
    }
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<(), Error> {
        for field in self.field_iter_mut() {
            if field.get_tag() == key {
                field.set_value_from_string(value)?;
                return Ok(());
            }
        }
        Err(Error::FieldNotFound {
            msg: key.to_string(),
        })
    }
    pub fn get_value(&self, key: &str) -> Result<String, Error> {
        for field in self.field_iter() {
            if field.get_tag() == key {
                return Ok(field.get_value_as_string());
            }
        }
        Err(Error::FieldNotFound {
            msg: key.to_string(),
        })
    }
}
