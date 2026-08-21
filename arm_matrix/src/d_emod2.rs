pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn d_emod2_mark() -> &'static str { \"d_emod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
