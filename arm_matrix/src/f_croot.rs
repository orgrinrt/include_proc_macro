pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn f_croot_mark() -> &'static str { \"f_croot\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
