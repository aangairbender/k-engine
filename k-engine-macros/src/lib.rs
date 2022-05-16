extern crate proc_macro;
extern crate syn;
extern crate quote;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(NodeData)]
pub fn derive_node_data(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    (quote::quote! {
        impl NodeData for #name {}
    }).into()
}