use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, Lit, Meta};

pub fn generate_validation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            _ => {
                return quote! {
                    compile_error!("Validate only works on structs with named fields");
                }
                .into()
            },
        },
        _ => {
            return quote! {
                compile_error!("Validate can only be derived for structs");
            }
            .into()
        },
    };

    let validation_checks = fields.named.iter().map(|field| {
        let field_name = &field.ident;
        let mut checks = Vec::new();

        // Process field attributes
        for attr in &field.attrs {
            if attr.path().is_ident("required") {
                checks.push(quote! {
                            if self.#field_name.is_none() {
                                errors.push(format!("Field {} is required", stringify!(#field_name)));
                            }
                        });
            } else if attr.path().is_ident("length") {
                if let Ok(meta_list) = attr.parse_args_with(|input: syn::parse::ParseStream| {
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated(input)
                }) {
                    // Store literals directly
                    let mut min_val = None;
                    let mut max_val = None;

                    for item in meta_list {
                        if let Meta::NameValue(name_value) = item {
                            if name_value.path.is_ident("min") {
                                if let Expr::Lit(expr_lit) = &name_value.value {
                                    if let Lit::Int(int_lit) = &expr_lit.lit {
                                        // Store the parsed value directly
                                        if let Ok(value) = int_lit.base10_parse::<usize>() {
                                            min_val = Some(value);
                                        }
                                    }
                                }
                            } else if name_value.path.is_ident("max") {
                                if let Expr::Lit(expr_lit) = &name_value.value {
                                    if let Lit::Int(int_lit) = &expr_lit.lit {
                                        if let Ok(value) = int_lit.base10_parse::<usize>() {
                                            max_val = Some(value);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(min) = min_val {
                        checks.push(quote! {
                                    if let Some(val) = &self.#field_name {
                                        if val.len() < #min {
                                            errors.push(format!("Field {} is too short", stringify!(#field_name)));
                                        }
                                    }
                                });
                    }

                    if let Some(max) = max_val {
                        checks.push(quote! {
                                    if let Some(val) = &self.#field_name {
                                        if val.len() > #max {
                                            errors.push(format!("Field {} is too long", stringify!(#field_name)));
                                        }
                                    }
                                });
                    }
                }
            } else if attr.path().is_ident("range") {
                if let Ok(meta_list) = attr.parse_args_with(|input: syn::parse::ParseStream| {
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated(input)
                }) {
                    // Store literals directly
                    let mut min_val = None;
                    let mut max_val = None;

                    for item in meta_list {
                        if let Meta::NameValue(name_value) = item {
                            if name_value.path.is_ident("min") {
                                if let Expr::Lit(expr_lit) = &name_value.value {
                                    if let Lit::Int(int_lit) = &expr_lit.lit {
                                        if let Ok(value) = int_lit.base10_parse::<i32>() {
                                            min_val = Some(value);
                                        }
                                    }
                                }
                            } else if name_value.path.is_ident("max") {
                                if let Expr::Lit(expr_lit) = &name_value.value {
                                    if let Lit::Int(int_lit) = &expr_lit.lit {
                                        if let Ok(value) = int_lit.base10_parse::<i32>() {
                                            max_val = Some(value);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(min) = min_val {
                        checks.push(quote! {
                                    if let Some(val) = self.#field_name {
                                        if val < #min {
                                            errors.push(format!("Field {} is below minimum value", stringify!(#field_name)));
                                        }
                                    }
                                });
                    }

                    if let Some(max) = max_val {
                        checks.push(quote! {
                                    if let Some(val) = self.#field_name {
                                        if val > #max {
                                            errors.push(format!("Field {} exceeds maximum value", stringify!(#field_name)));
                                        }
                                    }
                                });
                    }
                }
            }
        }

        quote! {
                    #(#checks)*
                }
    });

    let output = quote! {
        impl Validate for #name {
            fn validate(&self) -> Result<(), Vec<String>> {
                let mut errors = Vec::new();

                #(#validation_checks)*

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };

    output.into()
}
