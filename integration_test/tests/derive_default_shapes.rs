//! `DefaultImpl` against every struct shape, and against generics.
//!
//! It once wrote a fixed field name into the impl, so it worked on exactly one struct: a
//! named struct whose single field was called `field`. Anything else failed to compile
//! with the field name the macro had invented. These pin the shapes it accepts.

use examples::DefaultImpl;

#[derive(DefaultImpl, Debug, PartialEq)]
struct Named {
    retries: i32,
    label: String,
    enabled: bool,
}

#[derive(DefaultImpl, Debug, PartialEq)]
struct Tuple(u8, i64, String);

#[derive(DefaultImpl, Debug, PartialEq)]
struct Unit;

#[derive(DefaultImpl, Debug, PartialEq)]
struct OneField {
    only: u32,
}

#[derive(DefaultImpl, Debug, PartialEq)]
struct Generic<T> {
    held: T,
    count: usize,
}

#[derive(DefaultImpl, Debug, PartialEq)]
struct Bounded<T: Clone> {
    held: T,
}

#[test]
fn named_fields_default_individually() {
    assert_eq!(
        Named::default(),
        Named { retries: 0, label: String::new(), enabled: false }
    );
}

#[test]
fn a_tuple_struct_defaults_positionally() {
    assert_eq!(Tuple::default(), Tuple(0, 0, String::new()));
}

#[test]
fn a_unit_struct_has_a_default() {
    assert_eq!(Unit::default(), Unit);
}

#[test]
fn one_field_is_not_a_special_case() {
    // The shape the old implementation happened to work on, minus the field name it
    // hardcoded. It failing here is what the hardcoding looked like from outside.
    assert_eq!(OneField::default(), OneField { only: 0 });
}

#[test]
fn generics_carry_through_to_the_impl() {
    let d: Generic<i16> = Generic::default();
    assert_eq!(d, Generic { held: 0, count: 0 });

    let nested: Generic<Vec<u8>> = Generic::default();
    assert!(nested.held.is_empty());
}

#[test]
fn a_bound_on_the_parameter_survives() {
    // `split_for_impl` puts the bound on the impl rather than dropping it or repeating
    // it in argument position, which is the difference between this compiling and not.
    assert_eq!(Bounded::<u8>::default(), Bounded { held: 0 });
}
