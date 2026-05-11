use crate::renderer::{FieldRenderer, FieldSetRenderer};
use std::collections::HashMap;

pub trait FormRenderer {
    fn render_form_pre(&self, form: &crate::Form) -> String;
    fn render_form_post(&self, form: &crate::Form) -> String;
    fn field_renderers(&self) -> &HashMap<String, std::sync::Arc<Box<dyn FieldRenderer>>>;
    fn field_renderers_mut(
        &mut self,
    ) -> &mut HashMap<String, std::sync::Arc<Box<dyn FieldRenderer>>>;
    fn add_field_renderer(
        &mut self,
        field: &crate::Field,
        renderer: std::sync::Arc<Box<dyn FieldRenderer>>,
    ) {
        self.field_renderers_mut()
            .insert(field.get_tag().to_string(), renderer);
    }
    fn get_default_field_renderer(
        &self,
        field: &crate::Field,
    ) -> std::sync::Arc<Box<dyn FieldRenderer>>;
    fn get_field_renderer(&self, field: &crate::Field) -> std::sync::Arc<Box<dyn FieldRenderer>> {
        match self.field_renderers().get(field.get_tag()) {
            Some(renderer) => renderer.clone(),
            None => self.get_default_field_renderer(field),
        }
    }
    fn fieldset_renderers(&self) -> &HashMap<String, std::sync::Arc<Box<dyn FieldSetRenderer>>>;
    fn fieldset_renderers_mut(
        &mut self,
    ) -> &mut HashMap<String, std::sync::Arc<Box<dyn FieldSetRenderer>>>;
    fn add_fieldset_renderer(
        &mut self,
        fieldset: &crate::FieldSet,
        renderer: std::sync::Arc<Box<dyn FieldSetRenderer>>,
    ) {
        self.fieldset_renderers_mut()
            .insert(fieldset.tag.to_string(), renderer);
    }
    fn get_default_fieldset_renderer(
        &self,
        fieldset: &crate::FieldSet,
    ) -> std::sync::Arc<Box<dyn FieldSetRenderer>>;
    fn get_fieldset_renderer(
        &self,
        fieldset: &crate::FieldSet,
    ) -> std::sync::Arc<Box<dyn FieldSetRenderer>> {
        match self.fieldset_renderers().get(fieldset.tag.as_str()) {
            Some(renderer) => renderer.clone(),
            None => self.get_default_fieldset_renderer(fieldset),
        }
    }

    fn render(&self, form: &crate::Form) -> String
    where
        Self: Sized,
    {
        let mut result = String::new();
        result.push_str(&self.render_form_pre(form));
        for fieldset in form.fieldsets.iter() {
            result.push_str(&self.get_fieldset_renderer(fieldset).render(
                form,
                self as &dyn FormRenderer,
                fieldset,
            ));
        }
        result.push_str(&self.render_form_post(form));
        result
    }
}
