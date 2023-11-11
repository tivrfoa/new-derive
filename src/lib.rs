use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Type};

fn get_name_type(f: &Field) -> (String, String) {
    let param_name = f.ident.as_ref().unwrap().to_string();
    match &f.ty {
        Type::Path(syn::TypePath { qself: None, path }) => {
            if let Some(syn::PathSegment { ident, .. }) = path.segments.last() {
                return (param_name, ident.to_string());
            }
        }
        _ => todo!(),
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
