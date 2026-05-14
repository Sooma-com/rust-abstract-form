use crate::FieldSet;
use crate::field::single_value::SingleValue;
use crate::fieldset::to_empty_fieldset::ToEmptyFieldSet;
use std::sync::Arc;

pub trait ToFieldSet {
    fn to_field_set(&self) -> FieldSet;
}

macro_rules! impl_to_field_set_for_primitive {
    ($($t:ty),*) => {
        $(
            impl ToFieldSet for $t {
                fn to_field_set(&self) -> FieldSet {
                    let mut result = FieldSet::new("".to_string(), "".to_string());
                    result
                        .controls
                        .push(Arc::new(Box::new(SingleValue::<$t> {
                            tag: "".to_string(),
                            label: "".to_string(),
                            value: self.clone(),
                            validations: vec![],
                        })));
                    result
                }
            }
        )*
    };
}
impl_to_field_set_for_primitive!(
    String, bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize
);
#[cfg(feature = "chrono")]
impl_to_field_set_for_primitive!(chrono::NaiveDate, chrono::NaiveTime, chrono::NaiveDateTime);

impl<T> ToFieldSet for Option<T>
where
    T: ToFieldSet + ToEmptyFieldSet,
{
    fn to_field_set(&self) -> FieldSet {
        match self {
            Some(value) => value.to_field_set(),
            None => <T as ToEmptyFieldSet>::to_empty_fieldset(),
        }
    }
}
