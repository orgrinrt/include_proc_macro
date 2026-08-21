pub fn if_croot(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn if_croot_mark() -> &'static str { \"if_croot\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
