use crate::{Validation, errors::Error};

pub trait Field: std::fmt::Debug + Send + Sync {
    fn inner_type_id(&self) -> std::any::TypeId;
    fn get_tag(&self) -> &str;
    fn get_tag_mut(&mut self) -> &mut String;
    fn get_label(&self) -> &str;
    fn set_label(&mut self, label: &str);
    fn get_value_as_string(&self) -> String;
    fn set_value_from_string(&mut self, value: &str) -> Result<(), Error>;
    fn get_validations(&self) -> &Vec<std::sync::Arc<Box<dyn Validation>>>;
    // fn get_validations_by_type<T: std::any::Any>(
    //     &self,
    // ) -> impl Iterator<Item = &std::sync::Arc<Box<dyn Validation>>>;
    // fn get_validations_by_type_mut<T: std::any::Any>(
    //     &mut self,
    // ) -> impl Iterator<Item = &mut std::sync::Arc<Box<dyn Validation>>>;
    fn get_validations_mut(&mut self) -> &mut Vec<std::sync::Arc<Box<dyn Validation>>>;
    fn add_validation(&mut self, validation: std::sync::Arc<Box<dyn Validation>>);
    fn validate(&self) -> Result<bool, Vec<String>>
    where
        Self: Sized,
    {
        let mut errors = Vec::<String>::new();
        for validation in self.get_validations() {
            if let Err(error) = validation.validate(self) {
                errors.push(error);
            }
        }
        if errors.is_empty() {
            Ok(true)
        } else {
            Err(errors)
        }
    }
    fn prepend_tag(&mut self, tag: &str) {
        if tag.is_empty() {
            return;
        }
        let current_tag = self.get_tag_mut();
        if current_tag.is_empty() {
            *current_tag = tag.to_string();
        } else {
            current_tag.insert(0, '.');
            current_tag.insert_str(0, tag);
        }
    }
}
pub fn get_validations_by_type<T: std::any::Any>(
    field: &std::sync::Arc<Box<dyn Field>>,
) -> impl Iterator<Item = &std::sync::Arc<Box<dyn Validation>>> {
    field.get_validations().iter().filter(|validation| {
        let validation_type_id = validation.get_type_id();
        let target_type_id = std::any::TypeId::of::<T>();
        validation_type_id == target_type_id
    })
}
pub fn get_validations_by_type_mut<T: std::any::Any>(
    field: &mut std::sync::Arc<Box<dyn Field>>,
) -> Result<impl Iterator<Item = &mut std::sync::Arc<Box<dyn Validation>>>, Error> {
    let tag = field.get_tag().to_string();
    let iterator = std::sync::Arc::get_mut(field)
        .ok_or(Error::FieldNotMutable { msg: tag })?
        .get_validations_mut()
        .iter_mut()
        .filter(|validation| {
            let validation_type_id = validation.get_type_id();
            let target_type_id = std::any::TypeId::of::<T>();
            validation_type_id == target_type_id
        });
    Ok(iterator)
}
