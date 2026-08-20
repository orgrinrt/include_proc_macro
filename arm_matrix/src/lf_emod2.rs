pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn lf_emod2_mark() -> &'static str { \"lf_emod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
