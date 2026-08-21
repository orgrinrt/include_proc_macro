pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn f_umod2_mark() -> &'static str { \"f_umod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
