include_proc_macro::macros!(
    function -> foo::bar,
    attribute(generate_documentation) -> attr_impl::gen_doc,
    derive(DefaultImpl) -> derive_impl::impl_default
);

mod existing_mod;
use existing_mod::fizzbuzz;
include_proc_macro::macros!(
    function(fizz) -> fizzbuzz,
);

include_proc_macro::macros!(
    function(greet) -> @"hello.rs"::hello,
    attribute(derive_debug) -> @"test/inner.rs"::attr_derive_debug,
    derive(DisplayImpl) -> @"test/subdir/subdir.rs"::generate_display_impl,
);
