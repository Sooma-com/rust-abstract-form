use crate::Field;

pub trait ToField {
    fn to_field(&self) -> Field;
}

impl ToField for bool {
    fn to_field(&self) -> Field {
        Field::Boolean(crate::field::Boolean::new(
            "".to_string(),
            "".to_string(),
            *self,
        ))
    }
}
macro_rules! impl_to_field_for_numeric {
    ($($t:ty),*) => {
        $(
            impl ToField for $t {
                fn to_field(&self) -> Field {
                    Field::Number(crate::field::Number::new(
                        "".to_string(),
                        "".to_string(),
                        *self as f64,
                    ))
                }
            }
        )*
    };
}
impl_to_field_for_numeric!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize);

impl ToField for String {
    fn to_field(&self) -> Field {
        Field::Text(crate::field::Text::new(
            "".to_string(),
            "".to_string(),
            self.clone(),
        ))
    }
}
impl ToField for str {
    fn to_field(&self) -> Field {
        Field::Text(crate::field::Text::new(
            "".to_string(),
            "".to_string(),
            self.to_string(),
        ))
    }
}
impl<INNER: ToField + Default> ToField for Option<INNER> {
    fn to_field(&self) -> Field {
        match self {
            Some(inner) => inner.to_field(),
            None => INNER::default().to_field(),
        }
    }
}
