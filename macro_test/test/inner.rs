pub fn attr_derive_debug(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_str = attr.to_string();
    let item_str = item.to_string();
    let result = format!("#[derive(Debug)] {}", item_str);
    result.parse().unwrap()
}
