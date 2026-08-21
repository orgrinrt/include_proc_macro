//! What `DefaultImpl` refuses, pinned.
//!
//! Two cases, and each is a direction a passing test cannot reach. The first is the
//! `Default` bound on a type parameter no field uses, which every positive case in
//! `derive_default_shapes.rs` is blind to. The second is the enum refusal, which existed
//! only as a `compile_error!` in the macro with nothing asserting it fires.

use std::fs;

/// How many cases this suite expects to find.
///
/// `trybuild` is given a glob, and a glob matching nothing is not an error, so without this
/// the suite would pass having checked no case at all. The sibling suite in
/// `arm_matrix_test` carries the same guard for the same reason, and that reason is a
/// generator that deleted a `tests/` directory once while the workspace stayed green.
const EXPECTED_CASES: usize = 2;

#[test]
fn every_refusal_still_refuses() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/ui");
    let found = fs::read_dir(dir)
        .expect("the ui directory")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|e| e == "rs"))
        .count();

    assert_eq!(
        found, EXPECTED_CASES,
        "the ui directory holds {found} cases where {EXPECTED_CASES} are expected. \
         Adding one means raising the number here; finding fewer means one went missing."
    );

    trybuild::TestCases::new().compile_fail(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/ui/*.rs"));
}
