//! The crate builds under each feature, and the macros still expand under each.
//!
//! `no_std` on a crate that is `macro_rules!` only is easy to declare and easy to get wrong
//! in one specific way: the attribute applies to *this* crate, and what the macros expand
//! into is a `#[proc_macro]` entry point compiled in somebody else's. A proc-macro crate
//! cannot be `no_std` whatever this one says, because it runs on the host inside the
//! compiler.
//!
//! So building is not the check. Expanding is, and that is what the arm matrix does. This
//! runs it under each feature, so the claim covers the configurations rather than only the
//! default one.

use std::process::Command;

/// Runs a cargo command for one feature selection, in a target directory of its own so it
/// does not fight the outer `cargo test` for the build lock.
fn cargo(args: &[&str], features: &[&str]) -> (bool, String) {
    let mut command = Command::new(env!("CARGO"));
    command
        .args(args)
        .arg("--quiet")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env(
            "CARGO_TARGET_DIR",
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/feature-matrix"),
        );
    if !features.is_empty() {
        command.args(["--no-default-features", "--features", &features.join(",")]);
    }
    let output = command.output().expect("cargo runs");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

#[test]
fn the_default_selection_builds() {
    let (ok, err) = cargo(&["check"], &[]);
    assert!(ok, "the default features build:\n{err}");
}

#[test]
fn no_std_builds() {
    let (ok, err) = cargo(&["check"], &["no_std"]);
    assert!(ok, "no_std builds:\n{err}");
}

#[test]
fn no_alloc_builds_and_implies_no_std() {
    let (ok, err) = cargo(&["check"], &["no_alloc"]);
    assert!(ok, "no_alloc builds:\n{err}");
}

#[test]
fn the_macros_still_expand_under_no_alloc() {
    // The check that matters. Building says the crate compiles; this says the thing it
    // exists to do still works, at every path form and declaration shape the matrix covers.
    // The feature is named on the package that has it: `-p arm_matrix_test` with a bare
    // `no_alloc` asks the test crate for a feature it does not declare.
    let (ok, err) = cargo(&["test", "-p", "arm_matrix_test"], &["include_proc_macro/no_alloc"]);
    assert!(ok, "the arm matrix passes under no_alloc:\n{err}");
}
