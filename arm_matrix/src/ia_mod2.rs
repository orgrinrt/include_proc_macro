pub fn ia_mod2(
    _a: proc_macro::TokenStream,
    _i: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    "pub fn ia_mod2_mark() -> &'static str { \"ia_mod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
