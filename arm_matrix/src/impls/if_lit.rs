pub fn if_lit(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn if_lit_mark() -> &'static str { \"if_lit\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
