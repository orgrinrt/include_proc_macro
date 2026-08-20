pub fn if_umod2(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn if_umod2_mark() -> &'static str { \"if_umod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
