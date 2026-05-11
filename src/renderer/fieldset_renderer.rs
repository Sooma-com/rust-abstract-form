use crate::renderer::FormRenderer;

pub trait FieldSetRenderer: Send + Sync {
    fn render_fieldset_pre(
        &self,
        form: &crate::Form,
        form_renderer: &dyn FormRenderer,
        fieldset: &crate::FieldSet,
    ) -> String;
    fn render_fieldset_post(
        &self,
        form: &crate::Form,
        form_renderer: &dyn FormRenderer,
        fieldset: &crate::FieldSet,
    ) -> String;
    fn render(
        &self,
        form: &crate::Form,
        form_renderer: &dyn FormRenderer,
        fieldset: &crate::FieldSet,
    ) -> String {
        let mut result = String::new();
        result.push_str(&self.render_fieldset_pre(form, form_renderer, fieldset));
        for field in fieldset.field_iter() {
            result.push_str(&form_renderer.get_field_renderer(field).render(
                form,
                form_renderer,
                fieldset,
                field,
            ));
        }
        result.push_str(&self.render_fieldset_post(form, form_renderer, fieldset));
        result
    }
}
