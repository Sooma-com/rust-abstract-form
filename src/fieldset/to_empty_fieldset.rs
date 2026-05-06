use crate::{Field, FieldSet, field};

pub trait ToEmptyFieldSet {
    fn to_empty_fieldset() -> FieldSet;
}

impl ToEmptyFieldSet for String {
    fn to_empty_fieldset() -> FieldSet {
        FieldSet {
            tag: "".to_string(),
            label: "".to_string(),
            controls: vec![Field::Text(field::Text::new(
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ))],
        }
    }
}

impl ToEmptyFieldSet for bool {
    fn to_empty_fieldset() -> FieldSet {
        FieldSet {
            tag: "".to_string(),
            label: "".to_string(),
            controls: vec![Field::Boolean(field::Boolean::new(
                "".to_string(),
                "".to_string(),
                false,
            ))],
        }
    }
}

macro_rules! impl_to_empty_fieldset_for_numeric {
    ($($t:ty),*) => {
        $(
            impl ToEmptyFieldSet for $t {
                fn to_empty_fieldset() -> FieldSet {
                    FieldSet {
                        tag: "".to_string(),
                        label: "".to_string(),
                        controls: vec![Field::Number(field::Number::new(
                            "".to_string(),
                            "".to_string(),
                            0.0,
                        ))],
                    }
                }
            }
        )*
    };
}

impl_to_empty_fieldset_for_numeric!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize);

impl<INNER: ToEmptyFieldSet> ToEmptyFieldSet for Option<INNER> {
    fn to_empty_fieldset() -> FieldSet {
        INNER::to_empty_fieldset()
    }
}

#[cfg(feature = "chrono")]
impl ToEmptyFieldSet for chrono::NaiveDate {
    fn to_empty_fieldset() -> FieldSet {
        FieldSet {
            tag: "".to_string(),
            label: "".to_string(),
            controls: vec![Field::Date(field::Date::new(
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ))],
        }
    }
}

#[cfg(feature = "chrono")]
impl ToEmptyFieldSet for chrono::NaiveTime {
    fn to_empty_fieldset() -> FieldSet {
        FieldSet {
            tag: "".to_string(),
            label: "".to_string(),
            controls: vec![Field::Time(field::Time::new(
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ))],
        }
    }
}

#[cfg(feature = "chrono")]
impl ToEmptyFieldSet for chrono::NaiveDateTime {
    fn to_empty_fieldset() -> FieldSet {
        FieldSet {
            tag: "".to_string(),
            label: "".to_string(),
            controls: vec![Field::DateTime(field::DateTime::new(
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ))],
        }
    }
}
