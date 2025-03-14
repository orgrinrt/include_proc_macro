pub fn generate_display_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_str = input.to_string();
    let type_name = extract_type_name(&input_str);
    let impl_code = format!(
        r#"
            impl ::std::fmt::Display for {0} {{
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {{
                    write!(f, "This is a {0}")
                }}
            }}
        "#,
        type_name
    );

    impl_code.parse().unwrap()
}

fn extract_type_name(input: &str) -> String {
    // Simple parser to extract struct or enum name (not robust for production)
    if let Some(struct_idx) = input.find("struct") {
        let from_struct = &input[struct_idx + 6..];
        let name_end = from_struct
            .find('{')
            .unwrap_or_else(|| from_struct.find(';').unwrap_or(from_struct.len()));
        from_struct[..name_end].trim().to_string()
    } else if let Some(enum_idx) = input.find("enum") {
        let from_enum = &input[enum_idx + 4..];
        let name_end = from_enum.find('{').unwrap_or(from_enum.len());
        from_enum[..name_end].trim().to_string()
    } else {
        "UnknownType".to_string()
    }
}
