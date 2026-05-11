use crate::Field;
pub mod closed_single_choice;
pub use closed_single_choice::ClosedSingleChoice;

pub trait Validation: std::fmt::Debug {
    fn validate(&self, value: &Field) -> Result<bool, String>;
}
