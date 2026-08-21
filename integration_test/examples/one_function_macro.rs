//! The smallest thing this crate does: one line declares a function-like proc macro.
//!
//! The declaration lives in the `examples` crate, because a `#[proc_macro]` entry point can
//! only exist in a crate with `proc-macro = true`. That crate's whole body for this macro is
//!
//! ```ignore
//! macros!(function(greet) -> "hello/mod.rs"::hello);
//! ```
//!
//! which names the exported macro, the file the implementation is in, and the function in
//! it. The alternative is roughly fifteen lines of entry point per macro, written the same
//! way every time.
//!
//! ```text
//! cargo run -p integration_test --example one_function_macro
//! ```

use examples::greet;

fn main() {
    // The macro was written as an ordinary function taking and returning a `TokenStream`,
    // in a file that is not lib.rs, and reached from there by a path.
    let greeting = greet!("World");
    println!("{greeting}");
    assert_eq!(greeting, "Hello, World");
}
