pub fn if_mod2(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn if_mod2_mark() -> &'static str { \"if_mod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
