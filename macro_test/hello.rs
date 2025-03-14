pub fn hello(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_str = input.to_string();
    let output = format!("\"Hello, {}\"", input_str.trim_matches('"'));
    output.parse().unwrap()
}
