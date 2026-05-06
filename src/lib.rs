pub mod form;
pub use form::Form;
pub mod fieldset;
pub use fieldset::FieldSet;
pub mod field;
pub use field::Field;
pub use field::ToField;

#[cfg(test)]
extern crate self as abstract_form;

#[cfg(test)]
mod tests {
    use super::*;
    use abstract_form_derive::ToEmptyFieldSet;
    use fieldset::ToEmptyFieldSet as _;

    #[derive(ToEmptyFieldSet)]
    struct UserProfile {
        name: String,
        age: i32,
        active: bool,
    }

    #[test]
    fn test_derive_to_empty_fieldset() {
        let fieldset = UserProfile::to_empty_fieldset();

        assert_eq!(fieldset.tag, "UserProfile");
        assert_eq!(fieldset.label, "");
        assert_eq!(fieldset.controls.len(), 3);

        assert_eq!(
            fieldset.controls[0],
            Field::Text(field::Text::new("name".to_string(), "".to_string(), "".to_string()))
        );
        assert_eq!(
            fieldset.controls[1],
            Field::Number(field::Number::new("age".to_string(), "".to_string(), 0.0))
        );
        assert_eq!(
            fieldset.controls[2],
            Field::Boolean(field::Boolean::new("active".to_string(), "".to_string(), false))
        );
    }

    #[derive(ToEmptyFieldSet)]
    struct Address {
        street: String,
        postal_code: String,
    }

    #[derive(ToEmptyFieldSet)]
    struct Company {
        name: String,
        address: Address,
    }

    #[derive(ToEmptyFieldSet)]
    struct WithSkipped {
        name: String,
        #[abstract_form(skip)]
        id: i32,
        active: bool,
    }

    #[test]
    fn test_skip_attribute_excludes_field() {
        let fieldset = WithSkipped::to_empty_fieldset();

        assert_eq!(fieldset.tag, "WithSkipped");
        assert_eq!(fieldset.controls.len(), 2);

        assert_eq!(
            fieldset.controls[0],
            Field::Text(field::Text::new("name".to_string(), "".to_string(), "".to_string()))
        );
        assert_eq!(
            fieldset.controls[1],
            Field::Boolean(field::Boolean::new("active".to_string(), "".to_string(), false))
        );
    }

    #[test]
    fn test_nested_struct_tag_composition() {
        let fieldset = Company::to_empty_fieldset();

        assert_eq!(fieldset.tag, "Company");
        assert_eq!(fieldset.controls.len(), 3);

        assert_eq!(
            fieldset.controls[0],
            Field::Text(field::Text::new("name".to_string(), "".to_string(), "".to_string()))
        );
        assert_eq!(
            fieldset.controls[1],
            Field::Text(field::Text::new("address.Address.street".to_string(), "".to_string(), "".to_string()))
        );
        assert_eq!(
            fieldset.controls[2],
            Field::Text(field::Text::new("address.Address.postal_code".to_string(), "".to_string(), "".to_string()))
        );
    }
}
