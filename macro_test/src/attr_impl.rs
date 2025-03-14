use proc_macro::TokenStream;

pub fn gen_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = attr.to_string();
    let item_str = item.to_string();

    let output = format!("/// {}\n{}", attr_str, item_str);
    output.parse().unwrap_or(item)
}
