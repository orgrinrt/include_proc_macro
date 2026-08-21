pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn ld_crel_mark() -> &'static str { \"ld_crel\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
