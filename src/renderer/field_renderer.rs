pub trait FieldRenderer: Send + Sync {
    fn render(
        &self,
        form: &crate::Form,
        form_renderer: &dyn super::FormRenderer,
        fieldset: &crate::FieldSet,
        field: &std::sync::Arc<Box<dyn crate::Field>>,
    ) -> String;
}
