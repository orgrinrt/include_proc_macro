pub fn if_emod2(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn if_emod2_mark() -> &'static str { \"if_emod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
