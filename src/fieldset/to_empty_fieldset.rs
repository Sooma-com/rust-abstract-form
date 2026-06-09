use crate::FieldSet;
use crate::field::single_value::SingleValue;
use std::sync::Arc;

pub trait ToEmptyFieldSet {
    fn to_empty_fieldset() -> FieldSet;
}

macro_rules! impl_to_empty_fieldset_for_primitive {
    ($($t:ty),*) => {
        $(
            impl ToEmptyFieldSet for $t {
                fn to_empty_fieldset() -> FieldSet {
                    FieldSet {
                        tag: "".to_string(),
                        label: "".to_string(),
                        controls: vec![
                            Arc::new(Box::new(SingleValue::<$t> {
                                tag: "".to_string(),
                                label: "".to_string(),
                                value: Default::default(),
                                validations: vec![],
                            })),
                        ]
                    }
                }
            }
        )*
    };
}
impl_to_empty_fieldset_for_primitive!(
    String, bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize
);
#[cfg(feature = "chrono")]
impl_to_empty_fieldset_for_primitive!(chrono::NaiveDate, chrono::NaiveTime, chrono::NaiveDateTime);

impl<T> ToEmptyFieldSet for Option<T>
where
    T: ToEmptyFieldSet,
{
    fn to_empty_fieldset() -> FieldSet {
        <T as ToEmptyFieldSet>::to_empty_fieldset()
    }
}
impl<T> ToEmptyFieldSet for Box<T>
where
    T: ToEmptyFieldSet,
{
    fn to_empty_fieldset() -> FieldSet {
        <T as ToEmptyFieldSet>::to_empty_fieldset()
    }
}
impl<T> ToEmptyFieldSet for std::rc::Rc<T>
where
    T: ToEmptyFieldSet,
{
    fn to_empty_fieldset() -> FieldSet {
        <T as ToEmptyFieldSet>::to_empty_fieldset()
    }
}
impl<T> ToEmptyFieldSet for std::sync::Arc<T>
where
    T: ToEmptyFieldSet,
{
    fn to_empty_fieldset() -> FieldSet {
        <T as ToEmptyFieldSet>::to_empty_fieldset()
    }
}
impl<T> ToEmptyFieldSet for std::cell::Cell<T>
where
    T: ToEmptyFieldSet,
{
    fn to_empty_fieldset() -> FieldSet {
        <T as ToEmptyFieldSet>::to_empty_fieldset()
    }
}
impl<T> ToEmptyFieldSet for std::cell::RefCell<T>
where
    T: ToEmptyFieldSet,
{
    fn to_empty_fieldset() -> FieldSet {
        <T as ToEmptyFieldSet>::to_empty_fieldset()
    }
}
impl ToEmptyFieldSet for Vec<u8> {
    fn to_empty_fieldset() -> FieldSet {
        FieldSet {
            tag: "".to_string(),
            label: "".to_string(),
            controls: vec![Arc::new(Box::new(SingleValue::<String> {
                tag: "".to_string(),
                label: "".to_string(),
                value: "".to_string(),
                validations: vec![],
            }))],
        }
    }
}
#[cfg(feature = "hex_color")]
impl ToEmptyFieldSet for hex_color::HexColor {
    fn to_empty_fieldset() -> FieldSet {
        use crate::field::ConcreteSingleValue;

        FieldSet {
            tag: "".to_string(),
            label: "".to_string(),
            controls: vec![Arc::new(Box::new(ConcreteSingleValue::<
                hex_color::HexColor,
            > {
                tag: "".to_string(),
                label: "".to_string(),
                value: hex_color::HexColor::BLACK,
                validations: vec![],
            }))],
        }
    }
}
