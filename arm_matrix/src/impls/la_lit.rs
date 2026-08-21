pub fn imp(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn la_lit_mark() -> &'static str { \"la_lit\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
