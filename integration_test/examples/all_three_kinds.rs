//! The three kinds of procedural macro, declared the same way.
//!
//! Function-like, attribute and derive differ in the attribute they carry, the arguments
//! they take and how the implementation is called. Nothing else about them differs, and in
//! particular the path grammar naming the implementation is identical for all three:
//!
//! ```ignore
//! macros!(
//!     function(fizz) -> use fizzbuzz,
//!     attribute(derive_debug) -> @"test/inner.rs"::attr_derive_debug,
//!     derive(DisplayImpl) -> @"test/subdir/subdir.rs"::generate_display_impl,
//! );
//! ```
//!
//! ```text
//! cargo run -p integration_test --example all_three_kinds
//! ```

use examples::{derive_debug, fizz, DisplayImpl};

// An attribute macro, applied to an item.
#[derive_debug]
struct Counted {
    hits: u32,
}

// A derive macro, on a struct.
#[derive(DisplayImpl)]
struct Named;

fn main() {
    // Function-like, called with `!` like any other macro.
    println!("3 is {}", fizz!(3));
    println!("5 is {}", fizz!(5));
    println!("15 is {}", fizz!(15));
    println!("7 is {}", fizz!(7));

    // The attribute wrote the `Debug` impl.
    println!("{:?}", Counted { hits: 3 });

    // The derive wrote the `Display` impl.
    println!("{Named}");
}
