mod comprehension;
mod ident_meta;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn useless(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(IdentMeta)]
pub fn ident_meta(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    ident_meta::impl_ident_meta(ast)
}

#[proc_macro]
pub fn comp(input: TokenStream) -> TokenStream {
    use comprehension::Comp;
    let item = parse_macro_input!(input as Comp);
    quote::quote! { #item }.into()
}
