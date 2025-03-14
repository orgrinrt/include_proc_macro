#![cfg(test)]

use macro_test::*;
use std::fmt::Display;

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
