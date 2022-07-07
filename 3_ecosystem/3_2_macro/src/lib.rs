extern crate proc_macro;

struct Pair {
    key: syn::Expr,
    _arrow: syn::Token![=>],
    value: syn::Expr,
}

impl syn::parse::Parse for Pair {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _arrow: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct BTreeMapInput {
    pairs: syn::punctuated::Punctuated<Pair, syn::Token![,]>,
}

impl syn::parse::Parse for BTreeMapInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pairs: input.parse_terminated(Pair::parse)?,
        })
    }
}

#[proc_macro]
pub fn btreemap(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as BTreeMapInput);
    let pairs = input.pairs.into_iter().map(|Pair { key, value, .. }| {
        quote::quote! { (#key, #value) }
    });

    quote::quote! { std::collections::BTreeMap::from([#(#pairs),*]) }.into()
}
