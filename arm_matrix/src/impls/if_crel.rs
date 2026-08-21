pub fn if_crel(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn if_crel_mark() -> &'static str { \"if_crel\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
