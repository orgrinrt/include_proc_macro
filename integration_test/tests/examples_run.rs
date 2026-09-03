//! The examples are built by `cargo test` and never run by it, so they are run here.
//!
//! An example that compiles and then panics, or prints the wrong thing, is an example
//! that lies to whoever copies it. Each one is run and its output checked against what
//! the example itself claims it prints.

use std::process::Command;

/// Runs one example and returns what it printed, failing the test if it did not exit
/// zero.
fn run_example(name: &str) -> String {
    let output = Command::new(env!("CARGO"))
        .args(["run", "-q", "-p", "integration_test", "--example", name])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        // A separate target directory, so the outer `cargo test` holding the build lock
        // does not deadlock against this one.
        .env("CARGO_TARGET_DIR", concat!(env!("CARGO_MANIFEST_DIR"), "/target/examples"))
        .output()
        .unwrap_or_else(|e| panic!("could not run example {name}: {e}"));

    assert!(
        output.status.success(),
        "example {name} exited {}\n--- stderr\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );

    String::from_utf8(output.stdout).expect("example printed something that is not utf-8")
}

#[test]
fn one_function_macro_greets() {
    assert_eq!(run_example("one_function_macro").trim(), "Hello, World");
}

#[test]
fn all_three_kinds_reaches_every_kind() {
    let out = run_example("all_three_kinds");

    // A function-like macro, on each of the three interesting inputs.
    assert!(
        out.contains("3 is Fizz"),
        "no function-like output in:\n{out}"
    );
    assert!(
        out.contains("5 is Buzz"),
        "no function-like output in:\n{out}"
    );
    assert!(
        out.contains("15 is FizzBuzz"),
        "no function-like output in:\n{out}"
    );

    // An attribute macro, which added the `Debug` this line prints through.
    assert!(
        out.contains("Counted { hits: 3 }"),
        "no attribute output in:\n{out}"
    );

    // A derive, which wrote the `Display` impl.
    assert!(
        out.contains("This is a Named"),
        "no derive output in:\n{out}"
    );
}

#[test]
fn every_path_form_reaches_all_ten_forms() {
    let out = run_example("every_path_form");

    // One line per declaration form in the `examples` crate's `macros!` block.
    for expected in [
        "module path      : baz",                // function -> foo::bar
        "renamed          : ofo",                // function(ofo) -> use foo::baz
        "name in scope    : FizzBuzz",           // function(fizz) -> use fizzbuzz
        "file under src   : Hello, path forms",  // function(greet) -> "hello/mod.rs"::hello
        "derived default  : retries = 0",        // derive -> mod derive_impl::impl_default
        "derived display  : This is a Mode",     // derive -> @"test/subdir/subdir.rs"
        "validate present : true",               // derive with three helper attributes
        "node category    : leaf",               // derive with one helper attribute
        "documented const : 3",                  // attribute -> mod attr_impl::gen_doc
        "derive added     : Traced { step: 2 }", // attribute -> @"test/inner.rs"
    ] {
        assert!(out.contains(expected), "missing {expected:?} in:\n{out}");
    }
}
