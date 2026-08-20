pub mod b {
    pub mod c {
        pub fn ia_unest(_a: proc_macro::TokenStream, _i: proc_macro::TokenStream) -> proc_macro::TokenStream {
            "pub fn ia_unest_mark() -> &'static str { \"ia_unest\" }"
                .parse()
                .expect("the emitted item is a fixed, valid function")
        }
    }
}
