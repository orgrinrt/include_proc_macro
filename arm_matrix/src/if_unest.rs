pub mod b {
    pub mod c {
        pub fn if_unest(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
            "pub fn if_unest_mark() -> &'static str { \"if_unest\" }"
                .parse()
                .expect("the emitted item is a fixed, valid function")
        }
    }
}
