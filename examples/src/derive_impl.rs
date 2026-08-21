use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam};

/// Writes a `Default` impl that defaults every field.
///
/// Named, tuple and unit structs all work. Anything else is refused with a
/// `compile_error!`, because there is no sensible default for an enum without being told
/// which variant, and a union cannot have one at all.
///
/// Every type parameter picks up a `Default` bound, which is what the standard library's
/// own `derive(Default)` does and what makes a generic struct's impl compile at all.
pub fn impl_default(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();

    for param in &mut input.generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote!(::core::default::Default));
        }
    }
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let body = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let names = fields.named.iter().map(|f| &f.ident);
                quote! { #name { #(#names: ::core::default::Default::default()),* } }
            },
            Fields::Unnamed(fields) => {
                let defaults = fields
                    .unnamed
                    .iter()
                    .map(|_| quote! { ::core::default::Default::default() });
                quote! { #name(#(#defaults),*) }
            },
            Fields::Unit => quote! { #name },
        },
        _ => {
            return quote! {
                compile_error!("DefaultImpl can only be derived for structs");
            }
            .into()
        },
    };

    quote! {
        impl #impl_generics ::core::default::Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                #body
            }
        }
    }
    .into()
}
