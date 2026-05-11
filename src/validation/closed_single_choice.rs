#[derive(Clone, Debug)]
pub struct ClosedSingleChoice<T>
where
    T: std::fmt::Debug + Clone,
{
    pub message: Option<String>,
    pub options: Vec<T>,
}
impl<T> ClosedSingleChoice<T>
where
    T: std::fmt::Debug + Clone,
{
    pub fn new(options: Vec<T>) -> Self {
        Self {
            options,
            message: None,
        }
    }
}
macro_rules! impl_validation_for_string {
    ( $base:ident, $($generic:ty),* ) => {
        $(
            impl $crate::Validation for $base<$generic> {
                fn validate(&self, value: &$crate::Field) -> Result<bool, String> {
                    let string_value = value.get_value_as_string();
                    let options_as_strings = self.options.iter().map(|option| option.to_string()).collect::<Vec<String>>();
                    match options_as_strings.contains(&string_value) {
                        true => Ok(true),
                        false => Err(self.message.clone().unwrap_or("Invalid option".to_string())),
                    }
                }
            }
        )*
    };
}

macro_rules! impl_validation_for_numeric {
    ( $base:ident, $($generic:ty),* ) => {
        $(
            impl $crate::Validation for $base<$generic> {
                fn validate(&self, value: &$crate::Field) -> Result<bool, String> {
                    let current_value: $generic = match value {
                        crate::Field::Number(number) => Ok(number.value),
                        _ => Err("Invalid field type".to_string()),
                    }? as $generic;
                    match self.options.contains(&current_value) {
                        true => Ok(true),
                        false => Err(self.message.clone().unwrap_or("Invalid option".to_string())),
                    }
                }
            }
        )*
    };
}
impl_validation_for_string!(
    ClosedSingleChoice,
    String,
    chrono::NaiveDate,
    chrono::NaiveTime,
    chrono::NaiveDateTime
);
impl_validation_for_numeric!(
    ClosedSingleChoice,
    i8,
    i16,
    i32,
    i64,
    u8,
    u16,
    u32,
    u64,
    f32,
    f64,
    isize,
    usize
);
impl super::Validation for ClosedSingleChoice<bool> {
    fn validate(&self, value: &crate::Field) -> Result<bool, String> {
        let bool_value = match value {
            crate::Field::Boolean(boolean) => Ok(boolean.value),
            _ => Err("Invalid field type".to_string()),
        }?;
        match self.options.contains(&bool_value) {
            true => Ok(true),
            false => Err(self.message.clone().unwrap_or("Invalid option".to_string())),
        }
    }
}
