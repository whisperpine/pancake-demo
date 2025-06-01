use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn impl_ident_meta(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;
    let field_idents: Vec<Ident> = match ast.data {
        syn::Data::Struct(data_struct) => data_struct
            .fields
            .into_iter()
            .filter_map(|item| item.ident)
            .collect(),
        syn::Data::Enum(_) => panic!("Enums are not supported"),
        syn::Data::Union(_) => panic!("Unions are not supported"),
    };

    let field_ident_names: Vec<String> =
        field_idents.iter().map(|ident| ident.to_string()).collect();

    quote! {
        impl IdentMeta for #ident {
            fn get_name(&self) -> &'static str {
                stringify!(#ident)
            }
            fn get_items_name(&self) -> Vec<&'static str> {
                vec![#(#field_ident_names),*]
            }
        }
    }
    .into()
}
