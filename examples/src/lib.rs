mod derive_impl_with_attrs;
mod existing_mod;

use derive_impl_with_attrs::impl_with_attributes;
use existing_mod::fizzbuzz;
use include_proc_macro::macros;

macros!(
    function -> foo::bar,
    attribute(generate_documentation) -> mod attr_impl::gen_doc,
    derive(DefaultImpl) -> mod derive_impl::impl_default,
    derive(NodeTypeChecks, attributes(node_category)) -> use impl_with_attributes,
    derive(Validate, attributes(required, length, range)) -> derive_multiple_attrs::generate_validation,
    function(fizz) -> use fizzbuzz,
    function(greet) -> "hello/mod.rs"::hello,
    attribute(derive_debug) -> @"test/inner.rs"::attr_derive_debug,
    derive(DisplayImpl) -> @"test/subdir/subdir.rs"::generate_display_impl,
    function(ofo) -> use foo::baz
);
