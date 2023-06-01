use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(IdentMeta)]
pub fn ident_meta(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_ident_meta(ast)
}

fn impl_ident_meta(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;
    quote! {
        impl IdentMeta for #ident {
            fn get_name(&self) -> &'static str {
                stringify!(#ident)
            }
        }
    }
    .into()
}
