use proc_macro::TokenStream;

pub fn bar(input: TokenStream) -> TokenStream {
    let output = r#"
    pub fn bar() -> &'static str {
        "baz"
    }
    "#;
    output.parse().unwrap_or_else(|_| input)
}
