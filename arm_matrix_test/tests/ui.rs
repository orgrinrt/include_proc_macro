//! The refusals, pinned.
//!
//! Each of these is a shape the macros decline on purpose, and each declines with a
//! message saying what to write instead. Without this, loosening a matcher would
//! quietly restore the shape and every other test would still pass. Four of the
//! seven are the self-reference case, which crashed rustc outright with SIGBUS
//! before it was refused.

use std::fs;

/// How many cases this suite expects to find.
///
/// `trybuild` is given a glob, and a glob that matches nothing is not an error, so
/// without this the suite would pass having checked no case at all. That is not
/// hypothetical: the generator for the sibling matrix crate deleted this directory
/// once, and the workspace stayed green with every refusal unpinned.
const EXPECTED_CASES: usize = 7;

#[test]
fn every_refusal_still_refuses() {
    let found = fs::read_dir("tests/ui")
        .expect("the compile-fail case directory")
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|x| x == "rs"))
        .count();
    assert_eq!(
        found, EXPECTED_CASES,
        "expected {EXPECTED_CASES} compile-fail cases in tests/ui, found {found}. \
         Adding one means raising the constant; losing one means something deleted it."
    );

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
