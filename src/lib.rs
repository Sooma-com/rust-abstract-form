pub mod errors;
pub mod form;
pub use form::Form;
pub mod fieldset;
pub use fieldset::FieldSet;
pub mod field;
pub use field::Field;
pub use field::ToField;
pub mod validation;
pub use validation::Validation;

#[cfg(test)]
extern crate self as abstract_form;

#[cfg(test)]
mod tests {
    use super::*;
    use abstract_form_derive::{ToEmptyFieldSet, ToFieldSet};
    use fieldset::ToEmptyFieldSet as _;
    use fieldset::ToFieldSet as _;

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
            Field::Text(field::Text::new(
                "name".to_string(),
                "".to_string(),
                "".to_string()
            ))
        );
        assert_eq!(
            fieldset.controls[1],
            Field::Number(field::Number::new("age".to_string(), "".to_string(), 0.0))
        );
        assert_eq!(
            fieldset.controls[2],
            Field::Boolean(field::Boolean::new(
                "active".to_string(),
                "".to_string(),
                false
            ))
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
            Field::Text(field::Text::new(
                "name".to_string(),
                "".to_string(),
                "".to_string()
            ))
        );
        assert_eq!(
            fieldset.controls[1],
            Field::Boolean(field::Boolean::new(
                "active".to_string(),
                "".to_string(),
                false
            ))
        );
    }

    #[test]
    fn test_nested_struct_tag_composition() {
        let fieldset = Company::to_empty_fieldset();

        assert_eq!(fieldset.tag, "Company");
        assert_eq!(fieldset.controls.len(), 3);

        assert_eq!(
            fieldset.controls[0],
            Field::Text(field::Text::new(
                "name".to_string(),
                "".to_string(),
                "".to_string()
            ))
        );
        assert_eq!(
            fieldset.controls[1],
            Field::Text(field::Text::new(
                "address.Address.street".to_string(),
                "".to_string(),
                "".to_string()
            ))
        );
        assert_eq!(
            fieldset.controls[2],
            Field::Text(field::Text::new(
                "address.Address.postal_code".to_string(),
                "".to_string(),
                "".to_string()
            ))
        );
    }

    #[derive(ToFieldSet)]
    struct Person {
        name: String,
        age: i32,
        active: bool,
    }

    #[test]
    fn test_derive_to_field_set_with_values() {
        let person = Person {
            name: "Alice".to_string(),
            age: 30,
            active: true,
        };
        let fieldset = person.to_field_set();

        assert_eq!(fieldset.tag, "Person");
        assert_eq!(fieldset.label, "");
        assert_eq!(fieldset.controls.len(), 3);

        assert_eq!(
            fieldset.controls[0],
            Field::Text(field::Text::new(
                "name".to_string(),
                "".to_string(),
                "Alice".to_string()
            ))
        );
        assert_eq!(
            fieldset.controls[1],
            Field::Number(field::Number::new("age".to_string(), "".to_string(), 30.0))
        );
        assert_eq!(
            fieldset.controls[2],
            Field::Boolean(field::Boolean::new(
                "active".to_string(),
                "".to_string(),
                true
            ))
        );
    }

    #[derive(ToFieldSet)]
    struct WithSkippedFieldSet {
        name: String,
        #[abstract_form(skip)]
        id: i32,
        active: bool,
    }

    #[test]
    fn test_to_field_set_skip_attribute() {
        let item = WithSkippedFieldSet {
            name: "Bob".to_string(),
            id: 42,
            active: false,
        };
        let fieldset = item.to_field_set();

        assert_eq!(fieldset.tag, "WithSkippedFieldSet");
        assert_eq!(fieldset.controls.len(), 2);

        assert_eq!(
            fieldset.controls[0],
            Field::Text(field::Text::new(
                "name".to_string(),
                "".to_string(),
                "Bob".to_string()
            ))
        );
        assert_eq!(
            fieldset.controls[1],
            Field::Boolean(field::Boolean::new(
                "active".to_string(),
                "".to_string(),
                false
            ))
        );
    }

    #[derive(
        Debug, Clone, Copy, PartialEq, Eq, Default, strum::EnumIter, ToEmptyFieldSet, ToFieldSet,
    )]
    enum Color {
        #[default]
        Red,
        Green,
        Blue,
    }
    impl std::fmt::Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Red => write!(f, "red"),
                Self::Green => write!(f, "green"),
                Self::Blue => write!(f, "blue"),
            }
        }
    }

    #[test]
    fn test_enum_to_empty_fieldset() {
        let fieldset = Color::to_empty_fieldset();

        assert_eq!(fieldset.tag, "Color");
        assert_eq!(fieldset.controls.len(), 1);

        assert_eq!(
            fieldset.controls[0],
            Field::Text(field::Text::new(
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ))
        );

        let validations = fieldset.controls[0].get_validations();
        assert_eq!(validations.len(), 1);
        assert!(validations[0]
            .validate(&Field::Text(field::Text::new(
                "".to_string(),
                "".to_string(),
                "red".to_string(),
            )))
            .is_ok());
        assert!(validations[0]
            .validate(&Field::Text(field::Text::new(
                "".to_string(),
                "".to_string(),
                "invalid".to_string(),
            )))
            .is_err());
    }

    #[test]
    fn test_enum_to_field_set() {
        let fieldset = Color::Green.to_field_set();

        assert_eq!(fieldset.tag, "Color");
        assert_eq!(fieldset.controls.len(), 1);

        assert_eq!(
            fieldset.controls[0],
            Field::Text(field::Text::new(
                "".to_string(),
                "".to_string(),
                "green".to_string(),
            ))
        );

        let validations = fieldset.controls[0].get_validations();
        assert_eq!(validations.len(), 1);
        assert!(validations[0]
            .validate(&Field::Text(field::Text::new(
                "".to_string(),
                "".to_string(),
                "blue".to_string(),
            )))
            .is_ok());
        assert!(validations[0]
            .validate(&Field::Text(field::Text::new(
                "".to_string(),
                "".to_string(),
                "yellow".to_string(),
            )))
            .is_err());
    }

    #[derive(ToEmptyFieldSet, ToFieldSet)]
    struct WithEnum {
        name: String,
        color: Color,
    }

    #[test]
    fn test_struct_with_enum_field_to_empty_fieldset() {
        let fieldset = WithEnum::to_empty_fieldset();

        assert_eq!(fieldset.tag, "WithEnum");
        assert_eq!(fieldset.controls.len(), 2);

        assert_eq!(
            fieldset.controls[0],
            Field::Text(field::Text::new(
                "name".to_string(),
                "".to_string(),
                "".to_string(),
            ))
        );
        assert_eq!(
            fieldset.controls[1],
            Field::Text(field::Text::new(
                "color.Color".to_string(),
                "".to_string(),
                "".to_string(),
            ))
        );

        let validations = fieldset.controls[1].get_validations();
        assert_eq!(validations.len(), 1);
    }

    #[test]
    fn test_struct_with_enum_field_to_field_set() {
        let item = WithEnum {
            name: "Alice".to_string(),
            color: Color::Blue,
        };
        let fieldset = item.to_field_set();

        assert_eq!(fieldset.tag, "WithEnum");
        assert_eq!(fieldset.controls.len(), 2);

        assert_eq!(
            fieldset.controls[0],
            Field::Text(field::Text::new(
                "name".to_string(),
                "".to_string(),
                "Alice".to_string(),
            ))
        );
        assert_eq!(
            fieldset.controls[1],
            Field::Text(field::Text::new(
                "color.Color".to_string(),
                "".to_string(),
                "blue".to_string(),
            ))
        );

        let validations = fieldset.controls[1].get_validations();
        assert_eq!(validations.len(), 1);
    }
}
