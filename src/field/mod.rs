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

use crate::errors::Error;

#[derive(Clone, Debug, PartialEq)]
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
}
