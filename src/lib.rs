mod token;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;
use crate::token::{new_args, tup_args, set_val, set_methods, get_methods, info, root_extract, get_vis_pub};

#[proc_macro_derive(BasicMethod, attributes(only,exclude))]
pub fn basic_method_derive(input: TokenStream) -> TokenStream {
    let dinput = parse_macro_input!(input as DeriveInput);
    let (attrs, _vis, ident, data) = root_extract(dinput);
    
    let vispub = get_vis_pub();

    if let syn::Data::Struct(syn::DataStruct {fields: syn::Fields::Named(fname), .. }) = data {
        let syn::FieldsNamed{ named, .. } = fname;
        
        let new_args = new_args(&named);
        let tuple_args = tup_args(&named);
        let new_setval = set_val(&named);

        let set_methods = set_methods(&named, vispub.clone());
        let get_methods = get_methods(&named, vispub.clone());
        let info = info(&attrs, vispub.clone());
    
        return quote!{
            impl #ident {
                #vispub fn new(#(#new_args),*) -> Self {
                    Self { #(#new_setval),* }
                }
                #info
                #vispub fn fields() -> ::std::vec::Vec<(&'static str, &'static str)> {
                    let mut v = vec![];
                    #(v.push(#tuple_args));*;
                    v
                }
                #(#set_methods)*
                #(#get_methods)*
            }
        }.into()
    }
    panic!("'{}' is not supported!", ident.to_string())
}

