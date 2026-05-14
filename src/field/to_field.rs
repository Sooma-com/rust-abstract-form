use crate::Field;

pub trait ToField
where
    Self: Sized,
{
    fn to_field<F>(&self) -> impl Field
    where
        F: Field;
}
