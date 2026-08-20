pub mod b {
    pub mod c {
        pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
            "pub fn ld_unest_mark() -> &'static str { \"ld_unest\" }"
                .parse()
                .expect("the emitted item is a fixed, valid function")
        }
    }
}
