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
impl ToField for i8 {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
impl ToField for i16 {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
impl ToField for i32 {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
impl ToField for i64 {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
impl ToField for u8 {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
impl ToField for u16 {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
impl ToField for u32 {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
impl ToField for u64 {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
impl ToField for f32 {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
impl ToField for f64 {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self,
        ))
    }
}
impl ToField for isize {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
impl ToField for usize {
    fn to_field(&self) -> Field {
        Field::Number(crate::field::Number::new(
            "".to_string(),
            "".to_string(),
            *self as f64,
        ))
    }
}
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
