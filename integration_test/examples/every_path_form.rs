//! Every path form the declaration grammar accepts, in one block.
//!
//! An implementation lives wherever is convenient and the declaration says where. The
//! `examples` crate declares all ten of these in a single `macros!` invocation, and the
//! same ten forms are available to all three macro kinds, because the grammar is written
//! once and each kind supplies only what an item of that kind looks like:
//!
//! ```ignore
//! macros!(
//!     function -> foo::bar,                                    // module path, name from the path
//!     attribute(generate_documentation) -> mod attr_impl::gen_doc,  // declares the mod as well
//!     derive(DefaultImpl) -> mod derive_impl::impl_default,
//!     derive(NodeTypeChecks, attributes(node_category)) -> use impl_with_attributes,
//!     derive(Validate, attributes(required, length, range))
//!         -> derive_multiple_attrs::generate_validation,       // several helper attributes
//!     function(fizz) -> use fizzbuzz,                          // a name already in scope
//!     function(greet) -> "hello/mod.rs"::hello,                // a file, relative to src
//!     attribute(derive_debug) -> @"test/inner.rs"::attr_derive_debug,  // relative to crate root
//!     derive(DisplayImpl) -> @"test/subdir/subdir.rs"::generate_display_impl,
//!     function(ofo) -> use foo::baz                            // renamed, so two macros can
//! );                                                           // share one module
//! ```
//!
//! `mod` declares the module on the caller's behalf. `use` says the name is already in
//! scope. A quoted path is a file rather than a module, and `@` in front of it means the
//! file is found from the crate root rather than from `src`, which is what lets an
//! implementation sit outside the source tree entirely.
//!
//! ```text
//! cargo run -p integration_test --example every_path_form
//! ```

use examples::{
    derive_debug, fizz, generate_documentation, greet, DefaultImpl, DisplayImpl, NodeTypeChecks,
    Validate,
};

// Both of the attribute-owning derives below emit an impl of a trait the consumer
// declares, which is the ordinary shape for a derive: the macro writes the impl and the
// crate using it owns the trait.
trait Validate {
    fn validate(&self) -> Result<(), Vec<String>>;
}

trait NodeType {
    fn node_category() -> &'static str;
    fn has_category() -> bool;
}

// Form 5: a derive owning three helper attributes. The compiler accepts `required`,
// `length` and `range` on the fields because the declaration said the macro owns them;
// without that it rejects them as unknown before the macro ever runs.
#[derive(Validate)]
struct Registration {
    #[required]
    #[length(min = 3, max = 32)]
    name: Option<String>,
    #[range(min = 18, max = 130)]
    age: Option<i32>,
}

// Form 4: a derive owning one helper attribute, reached by a name already in scope.
#[derive(NodeTypeChecks)]
#[node_category("leaf")]
struct Leaf {
    depth: u8,
}

// Form 3: a derive whose module the declaration also declares.
#[derive(DefaultImpl)]
struct Settings {
    retries: i32,
}

// Form 9: a derive whose implementation is a file outside `src`.
#[derive(DisplayImpl)]
enum Mode {
    Fast,
}

// Form 2: an attribute macro. This one takes what it is given and puts it in a doc
// comment on the item it is attached to.
#[generate_documentation(the retry budget, in attempts)]
pub const RETRY_BUDGET: i32 = 3;

// Form 8: an attribute macro whose implementation is a file outside `src`. It adds a
// `#[derive(Debug)]` to whatever it is attached to.
#[derive_debug]
pub struct Traced {
    pub step: u8,
}

// Forms 1 and 10 expand to an item rather than to an expression, so they are invoked
// where an item goes. Both emit a function called `bar`, so each gets its own module,
// which is also the thing form 10 exists to show: `function(ofo)` renames the macro, so
// two macros can come out of one module without colliding.
mod from_module_path {
    examples::bar!();
}

mod renamed {
    examples::ofo!();
}

fn main() {
    // Forms 1 and 10, called through the functions they expanded into.
    println!("module path      : {}", from_module_path::bar());
    println!("renamed          : {}", renamed::bar());

    // Forms 6 and 7, which do expand to expressions.
    println!("name in scope    : {}", fizz!(15));
    println!("file under src   : {}", greet!("path forms"));

    // Forms 3 and 9.
    let settings = Settings::default();
    println!("derived default  : retries = {}", settings.retries);
    println!("derived display  : {}", Mode::Fast);

    // Forms 4 and 5.
    let registration = Registration { name: Some("orgrinrt".to_string()), age: Some(34) };
    println!("validate present : {}", registration.validate().is_ok());
    println!("node category    : {} (declared: {})", Leaf::node_category(), Leaf::has_category());
    println!("leaf depth       : {}", Leaf { depth: 0 }.depth);

    // Forms 2 and 8. The doc comment the attribute wrote is on the constant, and the
    // `Debug` the other one added is what prints here.
    println!("documented const : {}", RETRY_BUDGET);
    println!("derive added     : {:?}", Traced { step: 2 });
}
