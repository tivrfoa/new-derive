use quote::quote;

struct Struct {
    name: String,
    parameters: String,
}

impl Struct {
    fn parse(input: String) -> Self {
        let tmp: Vec<&str> = input.split_ascii_whitespace().collect();
        let name = tmp[1].into();
        let parameters = Struct::get_inner(&input);

        Self { name, parameters }
    }

    fn get_inner(input: &str) -> String {
        let l = input.find('{').unwrap() + 1;
        let r = {
            let mut r = 0;
            for i in (0..input.len()).rev() {
                if &input[i..i + 1] == "}" {
                    r = i;
                    break;
                }
            }
            r
        };
        input[l..r].into()
    }

    fn get_fields(&self) -> String {
        let mut ret = String::new();
        let params = self.parameters.replace("::", "");
        let tmp: Vec<&str> = params.split(':').collect();
        if tmp.is_empty() {
            return "".into();
        }
        ret.push_str(tmp[0]);
        for i in 1..tmp.len() - 1 {
            ret.push_str(", ");
            if tmp[i].contains(',') {
                let tmp2: Vec<&str> = tmp[i].split(',').collect();
                ret.push_str(tmp2.last().unwrap());
            } else {
                ret.push_str(tmp[i]);
            }
        }

        ret
    }
}

#[proc_macro_derive(New)]
pub fn derive_new(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let s = Struct::parse(input.to_string());
    let name: proc_macro2::TokenStream = s.name.parse().unwrap();
    let params: proc_macro2::TokenStream = s.parameters.parse().unwrap();
    let fields: proc_macro2::TokenStream = s.get_fields().parse().unwrap();

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
