use crate::{Field, FieldSet, errors::Error};
use std::sync::Arc;

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
    pub fn get_fieldset(&self, tag: &str) -> Option<&FieldSet> {
        self.fieldsets.iter().find(|fieldset| fieldset.tag == tag)
    }
    pub fn get_fieldset_mut(&mut self, tag: &str) -> Option<&mut FieldSet> {
        self.fieldsets
            .iter_mut()
            .find(|fieldset| fieldset.tag == tag)
    }
    pub fn remove_control(&mut self, control_tag: &str) -> Result<Arc<Box<dyn Field>>, Error> {
        for fieldset in &mut self.fieldsets {
            if let Some(control) = fieldset.remove_control(control_tag) {
                return Ok(control);
            }
        }
        Err(Error::FieldNotFound {
            msg: control_tag.to_string(),
        })
    }
    pub fn move_control(&mut self, fieldset_tag: &str, control_tag: &str) -> Result<(), Error> {
        let control = self.remove_control(control_tag)?;
        let target_fieldset =
            self.get_fieldset_mut(fieldset_tag)
                .ok_or_else(|| Error::FieldSetNotFound {
                    msg: fieldset_tag.to_string(),
                })?;
        target_fieldset.controls.push(control);
        Ok(())
    }
    pub fn add_control(&mut self, control: Arc<Box<dyn Field>>) -> &mut Self {
        if self.fieldsets.is_empty() {
            self.fieldsets
                .push(FieldSet::new("default".to_string(), "".to_string()));
        }
        self.fieldsets[0].controls.push(control);
        self
    }
    pub fn field_iter(&self) -> impl Iterator<Item = &Arc<Box<dyn Field>>> {
        self.fieldsets
            .iter()
            .flat_map(|fieldset| fieldset.field_iter())
    }
    pub fn field_iter_mut(&mut self) -> impl Iterator<Item = &mut Arc<Box<dyn Field>>> {
        self.fieldsets
            .iter_mut()
            .flat_map(|fieldset| fieldset.field_iter_mut())
    }
    pub fn get_field(&self, key: &str) -> Option<&Arc<Box<dyn Field>>> {
        self.field_iter().find(|&field| field.get_tag() == key)
    }
    pub fn get_field_mut(&mut self, key: &str) -> Option<&mut Arc<Box<dyn Field>>> {
        self.field_iter_mut().find(|field| field.get_tag() == key)
    }
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<(), Error> {
        for field in self.field_iter_mut() {
            if field.get_tag() == key {
                Arc::get_mut(field)
                    .ok_or_else(|| Error::FieldNotMutable {
                        msg: key.to_string(),
                    })?
                    .set_value_from_string(value)?;
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
