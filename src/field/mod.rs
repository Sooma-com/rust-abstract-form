pub mod text;
pub use text::Text;
pub mod number;
pub use number::Number;
pub mod date;
pub use date::Date;
pub mod time;
pub use time::Time;
pub mod date_time;
pub use date_time::DateTime;
pub mod boolean;
pub use boolean::Boolean;
pub mod to_field;
pub use to_field::ToField;

use crate::{Validation, errors::Error};

#[derive(Clone, Debug)]
pub enum Field {
    Text(Text),
    Number(Number),
    Date(Date),
    Time(Time),
    DateTime(DateTime),
    Boolean(Boolean),
}
impl Field {
    pub fn get_tag(&self) -> &str {
        match self {
            Field::Text(text) => &text.tag,
            Field::Number(number) => &number.tag,
            Field::Date(date) => &date.tag,
            Field::Time(time) => &time.tag,
            Field::DateTime(date_time) => &date_time.tag,
            Field::Boolean(boolean) => &boolean.tag,
        }
    }
    pub fn prepend_tag(&mut self, tag: &str) -> &mut Self {
        if tag.is_empty() {
            return self;
        }
        let mut current_tag = match self {
            Field::Text(text) => text.tag.clone(),
            Field::Number(number) => number.tag.clone(),
            Field::Date(date) => date.tag.clone(),
            Field::Time(time) => time.tag.clone(),
            Field::DateTime(date_time) => date_time.tag.clone(),
            Field::Boolean(boolean) => boolean.tag.clone(),
        };
        if current_tag.is_empty() {
            current_tag = tag.to_string();
        } else {
            current_tag.insert(0, '.');
            current_tag.insert_str(0, tag);
        }
        match self {
            Field::Text(text) => text.tag = current_tag,
            Field::Number(number) => number.tag = current_tag,
            Field::Date(date) => date.tag = current_tag,
            Field::Time(time) => time.tag = current_tag,
            Field::DateTime(date_time) => date_time.tag = current_tag,
            Field::Boolean(boolean) => boolean.tag = current_tag,
        }
        self
    }
    pub fn get_label(&self) -> &str {
        match self {
            Field::Text(text) => &text.label,
            Field::Number(number) => &number.label,
            Field::Date(date) => &date.label,
            Field::Time(time) => &time.label,
            Field::DateTime(date_time) => &date_time.label,
            Field::Boolean(boolean) => &boolean.label,
        }
    }
    pub fn get_value_as_string(&self) -> String {
        match self {
            Field::Text(text) => text.value.clone(),
            Field::Number(number) => number.value.to_string(),
            Field::Date(date) => date.value.clone(),
            Field::Time(time) => time.value.clone(),
            Field::DateTime(date_time) => date_time.value.clone(),
            Field::Boolean(boolean) => boolean.value.to_string(),
        }
    }
    pub fn set_value_from_string(&mut self, value: &str) -> Result<(), Error> {
        match self {
            Field::Text(text) => text.value = value.to_string(),
            Field::Number(number) => number.value = value.parse::<f64>()?,
            Field::Date(date) => date.value = value.to_string(),
            Field::Time(time) => time.value = value.to_string(),
            Field::DateTime(date_time) => date_time.value = value.to_string(),
            Field::Boolean(boolean) => boolean.value = value.parse::<bool>()?,
        };
        Ok(())
    }
    pub fn get_validations(&self) -> &Vec<std::sync::Arc<Box<dyn Validation>>> {
        match self {
            Field::Text(text) => text.get_validations(),
            Field::Number(number) => number.get_validations(),
            Field::Date(date) => date.get_validations(),
            Field::Time(time) => time.get_validations(),
            Field::DateTime(date_time) => date_time.get_validations(),
            Field::Boolean(boolean) => boolean.get_validations(),
        }
    }
    pub fn get_validations_mut(&mut self) -> &mut Vec<std::sync::Arc<Box<dyn Validation>>> {
        match self {
            Field::Text(text) => text.get_validations_mut(),
            Field::Number(number) => number.get_validations_mut(),
            Field::Date(date) => date.get_validations_mut(),
            Field::Time(time) => time.get_validations_mut(),
            Field::DateTime(date_time) => date_time.get_validations_mut(),
            Field::Boolean(boolean) => boolean.get_validations_mut(),
        }
    }
    pub fn add_validation(&mut self, validation: std::sync::Arc<Box<dyn Validation>>) {
        self.get_validations_mut().push(validation);
    }
    pub fn validate(&self) -> Result<bool, Vec<String>> {
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
}
impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.get_tag() == other.get_tag()
            && self.get_value_as_string() == other.get_value_as_string()
            && self.get_label() == other.get_label()
    }
}
