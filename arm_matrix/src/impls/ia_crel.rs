pub fn ia_crel(
    _a: proc_macro::TokenStream,
    _i: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    "pub fn ia_crel_mark() -> &'static str { \"ia_crel\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
