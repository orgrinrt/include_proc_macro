pub fn ia_croot(
    _a: proc_macro::TokenStream,
    _i: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    "pub fn ia_croot_mark() -> &'static str { \"ia_croot\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
