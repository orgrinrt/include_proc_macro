use proc_macro::TokenStream;

pub fn impl_default(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let type_name = extract_type_name(&input_str);

    let impl_code = format!(
        r#"
        impl ::std::default::Default for {0} {{
            fn default() -> Self {{
                {0} {{ 
                    field: ::std::default::Default::default()
                }}
            }}
        }}
    "#,
        type_name
    );

    impl_code.parse().unwrap()
}

fn extract_type_name(input: &str) -> String {
    input
        .split_whitespace()
        .skip_while(|s| *s != "struct")
        .nth(1)
        .unwrap_or("UnknownStruct")
        .to_string()
        .trim_end_matches('{')
        .to_string()
}
