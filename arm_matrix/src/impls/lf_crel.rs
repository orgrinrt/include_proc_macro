pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn lf_crel_mark() -> &'static str { \"lf_crel\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
