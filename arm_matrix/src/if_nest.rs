pub mod b {
    pub mod c {
        pub fn if_nest(_i: proc_macro::TokenStream) -> proc_macro::TokenStream {
            "pub fn if_nest_mark() -> &'static str { \"if_nest\" }"
                .parse()
                .expect("the emitted item is a fixed, valid function")
        }
    }
}
