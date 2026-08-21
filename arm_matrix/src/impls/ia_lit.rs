pub fn ia_lit(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn ia_lit_mark() -> &'static str { \"ia_lit\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
