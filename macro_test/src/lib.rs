mod existing_mod;
use existing_mod::fizzbuzz;
include_proc_macro::macros!(
    function -> foo::bar,
    attribute(generate_documentation) -> attr_impl::gen_doc,
    derive(DefaultImpl) -> derive_impl::impl_default,
    derive(NodeTypeChecks, attributes(node_category)) -> derive_impl_with_attrs::impl_with_attributes,
    derive(Validate, attributes(required, length, range)) -> derive_multiple_attrs::generate_validation,
    function(fizz) -> fizzbuzz,
    function(greet) -> @"hello.rs"::hello,
    attribute(derive_debug) -> @"test/inner.rs"::attr_derive_debug,
    derive(DisplayImpl) -> @"test/subdir/subdir.rs"::generate_display_impl,
);
