// comp: mapping for_if_clause
//
// mapping: expression
//
// for_if_clause:
//     | 'for' pattern 'in' expression ('if' expression)*
//
//  pattern: name (, name)*

#![cfg_attr(debug_assertions, allow(unused))]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};

pub(crate) struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

impl quote::ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern,
            sequence,
            conditions,
        } = &self.for_if_clause;

        tokens.extend(quote! {
            ::core::iter::IntoIterator::into_iter(#sequence).filter_map(move |#pattern| {
                (true #(&& (#conditions))*).then(|| #mapping)
            })
        })
    }
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
        })
    }
}

struct Mapping(syn::Expr);

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl quote::ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct ForIfClause {
    pattern: Pattern,
    sequence: syn::Expr,
    conditions: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![for]>()?;
        let pattern = input.parse::<Pattern>()?;
        _ = input.parse::<syn::Token![in]>()?;
        let sequence = input.parse::<syn::Expr>()?;
        let conditions = parse_zero_or_more(input);
        Ok(Self {
            pattern,
            sequence,
            conditions,
        })
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(item) = input.parse() {
        result.push(item);
    }
    result
}

struct Pattern(syn::Pat);

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.call(syn::Pat::parse_single).map(Self)
    }
}

impl quote::ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct Condition(syn::Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![if]>()?;
        input.parse().map(Self)
    }
}

impl quote::ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}
