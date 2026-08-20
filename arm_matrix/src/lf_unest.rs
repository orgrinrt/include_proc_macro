pub mod b {
    pub mod c {
        pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
            "pub fn lf_unest_mark() -> &'static str { \"lf_unest\" }"
                .parse()
                .expect("the emitted item is a fixed, valid function")
        }
    }
}
