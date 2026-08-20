pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn lda_crel_mark() -> &'static str { \"lda_crel\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
