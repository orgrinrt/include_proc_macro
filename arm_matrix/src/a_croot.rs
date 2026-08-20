pub fn imp(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn a_croot_mark() -> &'static str { \"a_croot\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
