pub fn ia_sroot(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn ia_sroot_mark() -> &'static str { \"ia_sroot\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
