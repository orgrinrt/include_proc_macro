pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "pub fn da_sroot_mark() -> &'static str { \"da_sroot\" }"
        .parse()
        .expect("the emitted item is a fixed, valid function")
}
