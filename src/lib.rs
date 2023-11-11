/*


https://github.com/napi-rs/napi-rs/blob/529317b5efe01e49137e8c42c6e90ca351805cce/crates/backend/src/typegen.rs#L9

*/
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, parse_macro_input, Type};

// #[proc_macro_derive(AnswerFn)]
// pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
//     "fn answer() -> u32 { 42 }".parse().unwrap()
// }

fn get_name_type(f: &Field) -> String {
	let mut ret = String::new();
	ret.push_str(&mut f.ident.as_ref().unwrap().to_string());
	ret.push(':');
	ret.push(' ');
	match &f.ty {
		Type::Path(syn::TypePath { qself: None, path }) => {
			if let Some(syn::PathSegment { ident, .. }) =
				path.segments.last() {
					ret.push_str(&mut ident.to_string());
				}
		}
		_ => todo!(),
	}
	ret
}

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;
	let mut parameters: Vec<String> = vec![];
	match input.data {
		Data::Struct(ref data) => {
			match data.fields {
				Fields::Named(ref fields) => {
					for f in &fields.named {
						parameters.push(get_name_type(f));
						// dbg!(&f.ident);
						// println!("{:?} has type {:?}", f.ident, f.ty);
						if let Some(f) = &f.ident {
							println!("field name: {}", f.to_string());
						}

						match &f.ty {
							Type::Path(syn::TypePath { qself: None, path }) => {
							  if let Some(syn::PathSegment { ident, .. }) =
								  path.segments.last() {
								let rust_ty = ident.to_string();
								println!("Type is: {}", rust_ty);
							  }
							}
							_ => todo!(),
						  }
					}
				}
				_ => todo!()
			}
		}
		_ => todo!()
	}
	// dbg!(parameters);
	let params: String = parameters.join(", ");
	dbg!(params);

    let expanded = quote! {
        // The generated impl.
        impl #name {
            fn answer(&self) -> usize {
                42
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}
