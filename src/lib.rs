use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Type};

fn get_name_type(f: &Field) -> (String, String) {
    let param_name = f.ident.as_ref().unwrap().to_string();
    match &f.ty {
        Type::Path(syn::TypePath { qself: None, path }) => {
            for p in &path.segments {
                println!("{}", p.ident.to_string());
                match &p.arguments {
                    syn::PathArguments::None => {
						println!("No args. Nothing to worry about?! =)");
					}
                    syn::PathArguments::AngleBracketed(args) => {
                        for arg in &args.args {
                            dbg!(arg);
                            match arg {
                                syn::GenericArgument::Lifetime(_) => todo!(),
                                syn::GenericArgument::Type(t) => {
                                    // println!("generic arg type is {}", t.);
                                    todo!("handle generic argument");
                                }
                                syn::GenericArgument::Const(_) => {
									println!("TODO handle Const?");
								}
                                syn::GenericArgument::AssocType(_) => todo!(),
                                syn::GenericArgument::AssocConst(_) => todo!(),
                                syn::GenericArgument::Constraint(_) => todo!(),
                                _ => {
									println!("TODO handle arg _");
								}
                            }
                        }
                    }
                    syn::PathArguments::Parenthesized(_) => todo!(),
                }
            }
            if let Some(syn::PathSegment { ident, arguments }) = path.segments.last() {
                // let (a, b, c) = arguments
                return (param_name, ident.to_string());
            }
        }
        Type::Path(syn::TypePath { qself, path }) => {
			println!("qself is some");
        }
		Type::Array(_) => {
			todo!("TODO handle Array");
		}
        _ => todo!("which type?!"),
    }
    todo!()
}

fn get_parameters(parameters: &[(String, String)]) -> String {
    let mut ret = String::new();
    for p in parameters {
        if !ret.is_empty() {
            ret.push_str(", ");
        }
        ret.push_str(&mut p.0.clone());
        ret.push_str(": ");
        ret.push_str(&mut p.1.clone());
    }
    ret
}

fn get_fields(parameters: &[(String, String)]) -> String {
    let mut ret = String::new();
    for p in parameters {
        if !ret.is_empty() {
            ret.push_str(", ");
        }
        ret.push_str(&mut p.0.clone());
    }
    ret
}

#[proc_macro_derive(New)]
pub fn derive_new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;
    let mut parameters: Vec<(String, String)> = vec![];
    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                for f in &fields.named {
                    parameters.push(get_name_type(f));
                }
            }
            _ => todo!(),
        },
        _ => todo!(),
    }
	dbg!(&parameters);

    let params: proc_macro2::TokenStream = get_parameters(&parameters).parse().unwrap();
    let fields: proc_macro2::TokenStream = get_fields(&parameters).parse().unwrap();

    let expanded = quote! {
        // The generated impl.
        impl #name {
            fn new(#params) -> Self {
                Self {
                    #fields
                }
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}
