use proc_macro::TokenStream;

pub fn impl_with_attributes(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let type_name = extract_type_name(&input_str);

    // Look for attribute markers in the input
    let has_category_attr = input_str.contains("#[node_category");

    // Extract any attribute values (simplified parsing)
    let category = if has_category_attr {
        let attr_content = input_str
            .split("#[node_category")
            .nth(1)
            .unwrap_or("")
            .split(']')
            .next()
            .unwrap_or("");

        attr_content
            .trim_start_matches('(')
            .trim_end_matches(')')
            .trim()
            .trim_matches('"')
    } else {
        "unknown"
    };

    let impl_code = format!(
        r#"
        // Generated code for {0} with node_category attributes
        impl NodeType for {0} {{
            fn node_category() -> &'static str {{
                "{1}"
            }}
            
            fn has_category() -> bool {{
                {2}
            }}
        }}
        "#,
        type_name, category, has_category_attr
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
