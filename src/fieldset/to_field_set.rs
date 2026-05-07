use crate::{Field, FieldSet};

pub trait ToFieldSet {
    fn to_field_set(&self) -> FieldSet;
}

impl ToFieldSet for String {
    fn to_field_set(&self) -> FieldSet {
        let mut result = FieldSet::new("".to_string(), "".to_string());
        result
            .controls
            .push(Field::Text(crate::field::text::Text::new(
                "".to_string(),
                "".to_string(),
                self.clone(),
            )));
        result
    }
}

impl ToFieldSet for bool {
    fn to_field_set(&self) -> FieldSet {
        let mut result = FieldSet::new("".to_string(), "".to_string());
        result
            .controls
            .push(Field::Boolean(crate::field::boolean::Boolean::new(
                "".to_string(),
                "".to_string(),
                *self,
            )));
        result
    }
}

macro_rules! impl_to_field_set_for_numeric {
    ($($t:ty),*) => {
        $(
            impl ToFieldSet for $t {
                fn to_field_set(&self) -> FieldSet {
                    let mut result = FieldSet::new("".to_string(), "".to_string());
                    result
                        .controls
                        .push(Field::Number(crate::field::number::Number::new(
                            "".to_string(),
                            "".to_string(),
                            *self as f64,
                        )));
                    result
                }
            }
        )*
    };
}

impl_to_field_set_for_numeric!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize);

impl<INNER> ToFieldSet for Option<INNER>
where
    INNER: ToFieldSet + Clone + Default,
{
    fn to_field_set(&self) -> FieldSet {
        self.clone().unwrap_or_default().to_field_set()
    }
}

#[cfg(feature = "chrono")]
impl ToFieldSet for chrono::NaiveDate {
    fn to_field_set(&self) -> FieldSet {
        let mut result = FieldSet::new("".to_string(), "".to_string());
        result
            .controls
            .push(Field::Date(crate::field::date::Date::new(
                "".to_string(),
                "".to_string(),
                self.to_string(),
            )));
        result
    }
}

#[cfg(feature = "chrono")]
impl ToFieldSet for chrono::NaiveTime {
    fn to_field_set(&self) -> FieldSet {
        let mut result = FieldSet::new("".to_string(), "".to_string());
        result
            .controls
            .push(Field::Date(crate::field::date::Date::new(
                "".to_string(),
                "".to_string(),
                self.to_string(),
            )));
        result
    }
}

#[cfg(feature = "chrono")]
impl ToFieldSet for chrono::NaiveDateTime {
    fn to_field_set(&self) -> FieldSet {
        let mut result = FieldSet::new("".to_string(), "".to_string());
        result
            .controls
            .push(Field::Date(crate::field::date::Date::new(
                "".to_string(),
                "".to_string(),
                self.to_string(),
            )));
        result
    }
}
