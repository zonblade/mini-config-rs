use proc_macro::TokenStream;

#[proc_macro_derive(Configure)]
pub fn arc_forge_macro(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote::quote! {

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let variant_path = std::any::type_name::<Self>()
                    .rsplit("::")
                    .next()
                    .unwrap();
                let enum_name = String::from(variant_path);
                write!(f, "{:?}::{:?}", enum_name, self)
            }
        }

        impl #name {
            pub fn get(&self) -> String {
                mini_config::arc::get_string::<#name>(self.to_owned()).to_string()
            }

            pub fn val(&self) -> String {
                mini_config::arc::get_string::<#name>(self.to_owned()).to_string()
            }

            pub fn set(&self,value:&str) -> () {
                mini_config::arc::set::<#name>(self.to_owned(), value)
            }

            pub fn get_str(&self) -> &'static str {
                let key = format!("{:}", self);
                let res = mini_config::arc::get_str(&key);
                res
            }
        }
    };
    gen.into()
}
