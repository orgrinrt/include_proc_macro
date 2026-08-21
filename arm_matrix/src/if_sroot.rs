pub fn if_sroot(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn if_sroot_mark() -> &'static str { \"if_sroot\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
