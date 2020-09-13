use proc_macro2::TokenStream;
use quote::quote;
use syn::*;

use crate::utils::*;

macro_rules! trace {
    ($($arg:expr),*) => {{
        #[cfg(feature = "trace")]
        println!($($arg),*);
    }};
}

pub fn handle_describe(input: TokenStream) -> TokenStream {
    trace!("handle_describe() starts");

    let DeriveInput { ident, data, .. } = parse2(input).expect("Unable to parse input");
    let ident_str = ident.to_string();
    trace!("Describing: {}", ident);

    let output = match data {
        Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                // ns: not skipped
                let ns: Vec<&Field> = named.iter().filter(|f| !is_skipped(f)).collect();

                let names = ns.iter().map(|f| {
                    f.ident
                        .clone()
                        .expect("All fields must be named")
                        .to_string()
                });
                let types = ns.iter().map(|f| &f.ty);

                quote! {
                    impl ::sbor::Describe for #ident {
                        fn describe() -> ::sbor::describe::Type {
                            use ::sbor::rust::vec::Vec;
                            use ::sbor::rust::string::ToString;
                            use ::sbor::Describe;

                            let mut named = Vec::new();
                            #(named.push((#names.to_string(), <#types>::describe()));)*

                            ::sbor::describe::Type::Struct {
                                name: #ident_str.to_string(),
                                fields: ::sbor::describe::Fields::Named { named },
                            }
                        }
                    }
                }
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let ns: Vec<&Field> = unnamed.iter().filter(|f| !is_skipped(f)).collect();

                let types = ns.iter().map(|f| &f.ty);

                quote! {
                    impl ::sbor::Describe for #ident {
                        fn describe() -> ::sbor::describe::Type {
                            use ::sbor::rust::string::ToString;
                            use ::sbor::rust::vec::Vec;
                            use ::sbor::Describe;

                            let mut unnamed = Vec::new();
                            #(unnamed.push(<#types>::describe());)*

                            ::sbor::describe::Type::Struct {
                                name: #ident_str.to_string(),
                                fields: ::sbor::describe::Fields::Unnamed { unnamed },
                            }
                        }
                    }
                }
            }
            syn::Fields::Unit => {
                quote! {
                    impl ::sbor::Describe for #ident {
                        fn describe() -> ::sbor::describe::Type {
                            use ::sbor::rust::string::ToString;

                            ::sbor::describe::Type::Struct {
                                name: #ident_str.to_string(),
                                fields: ::sbor::describe::Fields::Unit,
                            }
                        }
                    }
                }
            }
        },
        Data::Enum(DataEnum { variants, .. }) => {
            let names = variants.iter().map(|v| v.ident.to_string());
            let fields = variants.iter().map(|v| {
                let f = &v.fields;

                match f {
                    syn::Fields::Named(FieldsNamed { named, .. }) => {
                        let ns: Vec<&Field> = named.iter().filter(|f| !is_skipped(f)).collect();

                        let names = ns.iter().map(|f| {
                            f.ident
                                .clone()
                                .expect("All fields must be named")
                                .to_string()
                        });
                        let types = ns.iter().map(|f| &f.ty);

                        quote! {
                            {
                                let mut named = Vec::new();
                                #(named.push((#names.to_string(), <#types>::describe()));)*
                                ::sbor::describe::Fields::Named {
                                    named
                                }
                            }
                        }
                    }
                    syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let ns: Vec<&Field> = unnamed.iter().filter(|f| !is_skipped(f)).collect();

                        let types = ns.iter().map(|f| &f.ty);

                        quote! {
                            {
                                let mut unnamed = Vec::new();
                                #(unnamed.push(<#types>::describe());)*
                                ::sbor::describe::Fields::Unnamed {
                                    unnamed
                                }
                            }
                        }
                    }
                    syn::Fields::Unit => {
                        quote! {
                            {
                                ::sbor::describe::Fields::Unit
                            }
                        }
                    }
                }
            });

            quote! {
                impl ::sbor::Describe for #ident {
                    fn describe() -> ::sbor::describe::Type {
                        use ::sbor::rust::string::ToString;
                        use ::sbor::rust::vec::Vec;
                        use ::sbor::Describe;

                        let mut variants = Vec::new();
                        #(variants.push(::sbor::describe::Variant {
                            name: #names.to_string(),
                            fields: #fields
                        });)*

                        ::sbor::describe::Type::Enum {
                            name: #ident_str.to_string(),
                            variants,
                        }
                    }
                }
            }
        }
        Data::Union(_) => {
            panic!("Union is not supported!")
        }
    };
    trace!("handle_derive() finishes");

    #[cfg(feature = "trace")]
    crate::utils::print_compiled_code("Describe", &output);

    output
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use std::str::FromStr;

    use super::*;

    fn assert_code_eq(a: TokenStream, b: TokenStream) {
        assert_eq!(a.to_string(), b.to_string());
    }

    #[test]
    fn test_describe_struct() {
        let input = TokenStream::from_str("struct Test {a: u32}").unwrap();
        let output = handle_describe(input);

        assert_code_eq(
            output,
            quote! {
                impl ::sbor::Describe for Test {
                    fn describe() -> ::sbor::describe::Type {
                        use ::sbor::rust::vec::Vec;
                        use ::sbor::rust::string::ToString;
                        use ::sbor::Describe;
                        let mut named = Vec::new();
                        named.push(("a".to_string(), <u32>::describe()));
                        ::sbor::describe::Type::Struct {
                            name: "Test".to_string(),
                            fields: ::sbor::describe::Fields::Named { named },
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn test_describe_enum() {
        let input = TokenStream::from_str("enum Test {A, B (u32), C {x: u8}}").unwrap();
        let output = handle_describe(input);

        assert_code_eq(
            output,
            quote! {
                impl ::sbor::Describe for Test {
                    fn describe() -> ::sbor::describe::Type {
                        use ::sbor::rust::string::ToString;
                        use ::sbor::rust::vec::Vec;
                        use ::sbor::Describe;
                        let mut variants = Vec::new();
                        variants.push(::sbor::describe::Variant {
                            name: "A".to_string(),
                            fields: { ::sbor::describe::Fields::Unit }
                        });
                        variants.push(::sbor::describe::Variant {
                            name: "B".to_string(),
                            fields: {
                                let mut unnamed = Vec::new();
                                unnamed.push(<u32>::describe());
                                ::sbor::describe::Fields::Unnamed { unnamed }
                            }
                        });
                        variants.push(::sbor::describe::Variant {
                            name: "C".to_string(),
                            fields: {
                                let mut named = Vec::new();
                                named.push(("x".to_string(), <u8>::describe()));
                                ::sbor::describe::Fields::Named { named }
                            }
                        });
                        ::sbor::describe::Type::Enum {
                            name: "Test".to_string(),
                            variants,
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn test_skip_field_1() {
        let input = TokenStream::from_str("struct Test {#[sbor(skip)] a: u32}").unwrap();
        let output = handle_describe(input);

        assert_code_eq(
            output,
            quote! {
                impl ::sbor::Describe for Test {
                    fn describe() -> ::sbor::describe::Type {
                        use ::sbor::rust::vec::Vec;
                        use ::sbor::rust::string::ToString;
                        use ::sbor::Describe;
                        let mut named = Vec::new();
                        ::sbor::describe::Type::Struct {
                            name: "Test".to_string(),
                            fields: ::sbor::describe::Fields::Named { named },
                        }
                    }
                }
            },
        );
    }

    #[test]
    fn test_skip_field_2() {
        let input =
            TokenStream::from_str("enum Test {A, B (#[sbor(skip)] u32), C {#[sbor(skip)] x: u8}}")
                .unwrap();
        let output = handle_describe(input);

        assert_code_eq(
            output,
            quote! {
                impl ::sbor::Describe for Test {
                    fn describe() -> ::sbor::describe::Type {
                        use ::sbor::rust::string::ToString;
                        use ::sbor::rust::vec::Vec;
                        use ::sbor::Describe;
                        let mut variants = Vec::new();
                        variants.push(::sbor::describe::Variant {
                            name: "A".to_string(),
                            fields: { ::sbor::describe::Fields::Unit }
                        });
                        variants.push(::sbor::describe::Variant {
                            name: "B".to_string(),
                            fields: {
                                let mut unnamed = Vec::new();
                                ::sbor::describe::Fields::Unnamed { unnamed }
                            }
                        });
                        variants.push(::sbor::describe::Variant {
                            name: "C".to_string(),
                            fields: {
                                let mut named = Vec::new();
                                ::sbor::describe::Fields::Named { named }
                            }
                        });
                        ::sbor::describe::Type::Enum {
                            name: "Test".to_string(),
                            variants,
                        }
                    }
                }
            },
        );
    }
}
