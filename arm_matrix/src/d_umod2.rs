pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn d_umod2_mark() -> &'static str { \"d_umod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
