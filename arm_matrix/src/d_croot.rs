pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn d_croot_mark() -> &'static str { \"d_croot\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
