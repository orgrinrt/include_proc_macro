pub fn imp(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn la_mod2_mark() -> &'static str { \"la_mod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
