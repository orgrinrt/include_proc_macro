pub mod b {
    pub mod c {
        pub fn imp(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
            "pub fn lda_nest_mark() -> &'static str { \"lda_nest\" }"
                .parse()
                .expect("the emitted item is a fixed, valid function")
        }
    }
}
