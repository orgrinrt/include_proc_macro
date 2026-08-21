pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn da_crel_mark() -> &'static str { \"da_crel\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
