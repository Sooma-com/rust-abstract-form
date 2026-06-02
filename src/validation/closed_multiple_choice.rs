#[cfg(feature = "axum_gettext")]
use axum_gettext::ToLocalizedString;
#[cfg(feature = "axum_gettext")]
use std::str::FromStr;

use crate::{Field, Validation};

#[derive(Clone, Debug)]
pub struct ClosedMultipleChoice<T>
where
    T: std::fmt::Debug + std::fmt::Display + Clone + Send + Sync,
{
    pub message: Option<String>,
    pub options: Vec<(T, String)>,
}
impl<T> ClosedMultipleChoice<T>
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
impl<T> Validation for ClosedMultipleChoice<T>
where
    T: std::fmt::Debug + std::fmt::Display + Clone + Send + Sync + 'static,
{
    fn validate(&self, value: &dyn Field) -> Result<bool, String> {
        let string_values = value
            .get_value_as_string()
            .split(',')
            .map(str::to_string)
            .collect::<Vec<String>>();
        for string_value in string_values {
            if !self
                .options
                .iter()
                .any(|(value, _)| value.to_string() == string_value)
            {
                return Err(self
                    .message
                    .clone()
                    .unwrap_or(format!("Invalid option: {}", string_value)));
            }
        }
        Ok(true)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
