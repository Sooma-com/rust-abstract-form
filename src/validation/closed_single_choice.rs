#[cfg(feature = "axum_gettext")]
use axum_gettext::ToLocalizedString;
#[cfg(feature = "axum_gettext")]
use std::str::FromStr;

use crate::{Field, Validation};

#[derive(Clone, Debug)]
pub struct ClosedSingleChoice<T>
where
    T: std::fmt::Debug + std::fmt::Display + Clone + Send + Sync,
{
    pub message: Option<String>,
    pub options: Vec<(T, String)>,
}
impl<T> ClosedSingleChoice<T>
where
    T: std::fmt::Debug + std::fmt::Display + Clone + Send + Sync,
{
    pub fn new(options: Vec<(T, String)>) -> Self {
        Self {
            options,
            message: None,
        }
    }
    pub fn localize(&mut self, translated_options: impl IntoIterator<Item = (String, String)>) {
        for (untranslated_label, translated_label) in translated_options {
            if let Some(option) = self
                .options
                .iter_mut()
                .find(|(_, label)| label == &untranslated_label)
            {
                option.1 = translated_label;
            }
        }
    }
    pub fn filter(&mut self, filter: impl Fn(&T) -> bool) {
        self.options.retain(|(value, _)| filter(value));
    }

    #[cfg(feature = "axum_gettext")]
    pub fn localize_gettext<L>(&mut self, lang: &str)
    where
        L: ToLocalizedString + FromStr,
    {
        self.options.iter_mut().for_each(|(_, label)| {
            if let Ok(value) = L::from_str(label) {
                *label = value.to_localized_string(lang);
            }
        });
    }
}
impl<T> Validation for ClosedSingleChoice<T>
where
    T: std::fmt::Debug + std::fmt::Display + Clone + Send + Sync + 'static,
{
    fn validate(&self, value: &dyn Field) -> Result<bool, String> {
        let string_value = value.get_value_as_string();
        match self
            .options
            .iter()
            .any(|(value, _)| value.to_string() == string_value)
        {
            true => Ok(true),
            false => Err(self.message.clone().unwrap_or("Invalid option".to_string())),
        }
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// macro_rules! impl_validation_for_string {
//     ( $base:ident, $($generic:ty),* ) => {
//         $(
//             impl $crate::Validation for $base<$generic> {
//                 fn validate(&self, value: &$crate::Field) -> Result<bool, String> {
//                     let string_value = value.get_value_as_string();
//                     #[allow(clippy::cmp_owned)]
//                     match self.options.iter().any(|(value, _)| value.to_string() == string_value) {
//                         true => Ok(true),
//                         false => Err(self.message.clone().unwrap_or("Invalid option".to_string())),
//                     }
//                 }
//                 fn as_any(&self) -> &dyn std::any::Any {
//                     self
//                 }
//                 fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//                     self
//                 }
//             }
//         )*
//     };
// }

// macro_rules! impl_validation_for_numeric {
//     ( $base:ident, $($generic:ty),* ) => {
//         $(
//             impl $crate::Validation for $base<$generic> {
//                 fn validate(&self, value: &$crate::Field) -> Result<bool, String> {
//                     let current_value: $generic = match value {
//                         crate::Field::Number(number) => Ok(number.value),
//                         _ => Err("Invalid field type".to_string()),
//                     }? as $generic;
//                     match self.options.iter().any(|(value, _)| *value == current_value) {
//                         true => Ok(true),
//                         false => Err(self.message.clone().unwrap_or("Invalid option".to_string())),
//                     }
//                 }
//                 fn as_any(&self) -> &dyn std::any::Any {
//                     self
//                 }
//                 fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//                     self
//                 }
//             }
//         )*
//     };
// }
// impl_validation_for_string!(ClosedSingleChoice, String);
// #[cfg(feature = "chrono")]
// impl_validation_for_string!(
//     ClosedSingleChoice,
//     chrono::NaiveDate,
//     chrono::NaiveTime,
//     chrono::NaiveDateTime
// );
// impl_validation_for_numeric!(
//     ClosedSingleChoice,
//     i8,
//     i16,
//     i32,
//     i64,
//     u8,
//     u16,
//     u32,
//     u64,
//     f32,
//     f64,
//     isize,
//     usize
// );
// impl super::Validation for ClosedSingleChoice<bool> {
//     fn validate(&self, value: &crate::Field) -> Result<bool, String> {
//         let bool_value = match value {
//             crate::Field::Boolean(boolean) => Ok(boolean.value),
//             _ => Err("Invalid field type".to_string()),
//         }?;
//         match self.options.iter().any(|(value, _)| *value == bool_value) {
//             true => Ok(true),
//             false => Err(self.message.clone().unwrap_or("Invalid option".to_string())),
//         }
//     }
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }
//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//         self
//     }
// }
