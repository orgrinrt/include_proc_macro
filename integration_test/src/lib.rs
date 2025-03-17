#[cfg(test)]
mod tests {
    use examples::{
        derive_debug, fizz, generate_documentation, greet, DefaultImpl, DisplayImpl,
        NodeTypeChecks, Validate,
    };

    #[test]
    fn test_function_macros() {
        // greet
        let result = greet!("World");
        assert_eq!(result, "Hello, World");

        // fizzbuzz
        assert_eq!(fizz!(3), "Fizz");
        assert_eq!(fizz!(5), "Buzz");
        assert_eq!(fizz!(15), "FizzBuzz");
        assert_eq!(fizz!(7), 7);
    }

    #[test]
    fn test_derive_debug_attr_macro() {
        #[derive_debug]
        struct TestStruct {
            field: i32,
        }

        let test = TestStruct {
            field: 42,
        };

        let debug_str = format!("{:?}", test);
        assert!(debug_str.contains("TestStruct"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_gen_doc_attr_macro() {
        #[generate_documentation]
        struct TestStruct {
            field: String,
        }
        let test = TestStruct {
            field: "Hello".to_string(),
        };
        let doc_str = format!("generate_documentation {}", test.field);
        // TODO: how to test doc string?
    }

    #[test]
    fn test_derive_display_macros() {
        #[derive(DisplayImpl)]
        struct TestDisplay;

        let test = TestDisplay;
        assert_eq!(format!("{}", test), "This is a TestDisplay");

        #[derive(DisplayImpl)]
        enum TestEnum {
            A,
            B,
        }

        let test_enum = TestEnum::A;
        assert_eq!(format!("{}", test_enum), "This is a TestEnum");
    }

    #[test]
    fn test_derive_default_macros() {
        #[derive(DefaultImpl)]
        struct TestDefault {
            field: i32,
        }

        let test = TestDefault::default();
        assert_eq!(i32::default(), test.field);
    }

    #[test]
    fn test_derive_with_attrs() {
        trait NodeType {
            fn node_category() -> &'static str;
            fn has_category() -> bool;
        }

        #[derive(NodeTypeChecks)]
        struct RegularNode {
            data: String,
        }

        #[derive(NodeTypeChecks)]
        #[node_category("special")]
        struct SpecialNode {
            data: String,
        }

        // Test the regular node (without attributes)
        assert_eq!(RegularNode::node_category(), "unknown");
        assert_eq!(RegularNode::has_category(), false);

        // Test the special node (with node_category attribute)
        assert_eq!(SpecialNode::node_category(), "special");
        assert_eq!(SpecialNode::has_category(), true);
    }

    #[test]
    fn test_validation_derive() {
        trait Validate {
            fn validate(&self) -> Result<(), Vec<String>>;
        }

        #[derive(Validate)]
        struct User {
            #[required]
            name: Option<String>,

            #[length(min = 8, max = 64)]
            password: Option<String>,

            #[range(min = 13, max = 120)]
            age: Option<i32>,
        }

        // Valid user
        let valid_user = User {
            name: Some("John".to_string()),
            password: Some("secure_password".to_string()),
            age: Some(30),
        };
        assert!(valid_user.validate().is_ok());

        // Invalid user - missing required field
        let missing_name = User {
            name: None,
            password: Some("secure_password".to_string()),
            age: Some(30),
        };
        assert!(missing_name.validate().is_err());

        // Invalid user - password too short
        let short_password = User {
            name: Some("John".to_string()),
            password: Some("short".to_string()),
            age: Some(30),
        };
        assert!(short_password.validate().is_err());

        // Invalid user - age out of range
        let invalid_age = User {
            name: Some("John".to_string()),
            password: Some("secure_password".to_string()),
            age: Some(10),
        };
        assert!(invalid_age.validate().is_err());
    }
}
