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
