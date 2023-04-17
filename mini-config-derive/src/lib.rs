use proc_macro::TokenStream;

#[proc_macro_derive(Configure)]
pub fn arc_forge_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens as a Rust item (enum, struct, etc.)
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Generate the implementation of the get_str and get_json functions for the given enum
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
                mini_config::arc::set::<#name>(self.to_owned(), String::from(value))
            }
        }
    };

    // Return the generated code as a TokenStream
    gen.into()
}