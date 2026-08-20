pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn da_mod2_mark() -> &'static str { \"da_mod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
