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

#[test]
#[ignore = "catalogue: needs the 1.56.1 toolchain; run with --ignored"]
fn the_declared_minimum_toolchain_builds_the_crate() {
    // `rust-version = "1.56"` carried a comment saying it had been verified by expanding
    // every documented form under 1.56.1. Nothing pinned it, there is no CI, and a claim
    // resting on one hand check nobody can repeat is a claim about a moment rather than
    // about the crate.
    //
    // Marked ignore, because it needs a toolchain most machines do not have and a library's
    // `cargo test` should not fail for that. `cargo test -- --ignored` runs it, and
    // `rustup toolchain install 1.56.1` is what it needs.
    const MSRV: &str = "1.56.1";

    let installed = Command::new("rustup")
        .args(["toolchain", "list"])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).contains(MSRV))
        .unwrap_or(false);

    assert!(
        installed,
        "the {MSRV} toolchain is not installed, so the `rust-version` claim cannot be \
         checked here. `rustup toolchain install {MSRV}` and run again.",
    );

    // Built as a crate of its own rather than in place. The workspace's `Cargo.lock` is
    // format version 4, which 1.56's cargo refuses to parse, and that is a fact about the
    // lock file rather than about whether this crate's source compiles. This crate has no
    // dependencies at all, so a copy of the manifest and `src/` is the whole of it.
    let root = std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/target/msrv-crate"));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).expect("the msrv crate directory");

    std::fs::write(
        root.join("Cargo.toml"),
        format!(
            "[package]\nname = \"msrv_check\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\
             rust-version = \"{MSRV}\"\n\n[dependencies]\n\n[workspace]\n",
        ),
    )
    .expect("the msrv manifest");

    // The macro definitions, minus the `#![doc = include_str!(..)]` that would pull in a
    // README this copy does not have.
    let lib = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"))
        .expect("the crate source");
    let lib: String = lib.lines().filter(|line| !line.starts_with("#![doc")).collect::<Vec<_>>().join("\n");
    std::fs::write(root.join("src").join("lib.rs"), lib).expect("the msrv source");

    let output = Command::new("cargo")
        .args([format!("+{MSRV}"), "check".into(), "--quiet".into()])
        .current_dir(&root)
        .env("CARGO_TARGET_DIR", root.join("target"))
        .output()
        .expect("cargo runs");

    assert!(
        output.status.success(),
        "the macro definitions do not build under the declared minimum, {MSRV}:\n{}",
        String::from_utf8_lossy(&output.stderr),
    );
}
