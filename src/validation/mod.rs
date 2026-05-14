use std::any::TypeId;

use crate::Field;
pub mod closed_single_choice;
pub use closed_single_choice::ClosedSingleChoice;

pub trait Validation: std::fmt::Debug + Send + Sync
where
    Self: 'static,
{
    fn validate(&self, value: &dyn Field) -> Result<bool, String>;
    fn get_type_id(&self) -> TypeId {
        std::any::TypeId::of::<Self>()
    }
    fn get_type_name(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
