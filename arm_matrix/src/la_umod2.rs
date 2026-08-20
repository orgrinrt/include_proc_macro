pub fn imp(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn la_umod2_mark() -> &'static str { \"la_umod2\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
