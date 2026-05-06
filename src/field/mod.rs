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
            current_tag.insert_str(0, ".");
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
}
