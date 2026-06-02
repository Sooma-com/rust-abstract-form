use crate::{Field, Validation, errors::Error};

#[derive(Debug, Clone)]
pub struct ConcreteSingleValue<INNER> {
    pub tag: String,
    pub label: String,
    pub value: INNER,
    pub validations: Vec<std::sync::Arc<Box<dyn Validation>>>,
}
#[cfg(feature = "hex_color")]
impl Field for ConcreteSingleValue<hex_color::HexColor> {
    fn inner_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<hex_color::HexColor>()
    }

    fn get_tag(&self) -> &str {
        self.tag.as_str()
    }

    fn get_tag_mut(&mut self) -> &mut String {
        &mut self.tag
    }

    fn get_label(&self) -> &str {
        self.label.as_str()
    }

    fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    fn get_value_as_string(&self) -> String {
        hex_color::Display::new(self.value).to_string()
    }

    fn set_value_from_string(&mut self, value: &str) -> Result<(), crate::errors::Error> {
        match hex_color::HexColor::parse_rgb(value) {
            Ok(value) => self.value = value,
            Err(e) => return Err(Error::FormatConversionError { msg: e.to_string() }),
        }
        Ok(())
    }

    fn get_validations(&self) -> &Vec<std::sync::Arc<Box<dyn Validation>>> {
        &self.validations
    }

    fn get_validations_mut(&mut self) -> &mut Vec<std::sync::Arc<Box<dyn Validation>>> {
        &mut self.validations
    }

    fn add_validation(&mut self, validation: std::sync::Arc<Box<dyn Validation>>) {
        self.validations.push(validation);
    }
}
