mod token;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;
use token::{extract_unfield, set_methods_unnamed, get_methods_unnamed, enum_variants, from_trait, get_methods_mut};
use crate::token::{new_args, tup_args, set_val, set_methods, get_methods, info, root_extract, get_vis_pub, unamed_field, is_enum_field_unit};

#[proc_macro_derive(BasicMethod, attributes(only,exclude))]
pub fn basic_method_derive(input: TokenStream) -> TokenStream {
    let dinput = parse_macro_input!(input as DeriveInput);
    let (attrs, _vis, ident, data) = root_extract(dinput);
    
    let vispub = get_vis_pub();

    match data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fname), .. }) => {
            let syn::FieldsNamed{ named, .. } = fname;
            
            let new_args = new_args(&named);
            let tuple_args = tup_args(&named);
            let new_setval = set_val(&named);

            let set_methods = set_methods(&named, vispub.clone());
            let get_methods = get_methods(&named, vispub.clone());
            let get_methods_mut = get_methods_mut(&named, vispub.clone());
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
                    #(#get_methods_mut)*
                }
            }.into()
        }
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Unit, .. }) => {
            let info = info(&attrs, vispub.clone());
        
            return quote!{
                impl #ident {
                    #info
                }
            }.into()
        }
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Unnamed(unfield), .. }) => {
            let punc = extract_unfield(unfield);
            let fields = unamed_field(&punc, vispub.clone());
            let set_methods = set_methods_unnamed(&punc, vispub.clone());
            let get_methods = get_methods_unnamed(&punc, vispub.clone());
            let get_methods_mut = get_methods_mut(&punc, vispub.clone());
            let info = info(&attrs, vispub.clone());
        
            return quote!{
                impl #ident {
                    #info
                    #vispub fn fields() -> ::std::vec::Vec<(&'static str)> {
                        let mut v = vec![];
                        #(v.push(#fields));*;
                        v
                    }
                    #(#set_methods)*
                    #(#get_methods)*
                    #(#get_methods_mut)*
                }
            }.into()
        }
        syn::Data::Enum(dataenum) => {
            let variants = enum_variants(&dataenum.variants);
            let from_trait = from_trait(&dataenum.variants);
            let is_unit = is_enum_field_unit(&dataenum.variants);
            if is_unit {
                return quote!{
                    impl #ident {
                        #vispub fn variants() -> ::std::vec::Vec<&'static str> {
                            let mut v = vec![];
                            #(v.push(#variants));*;
                            v
                        }
                    }
                    impl ::std::convert::From<&str> for #ident {
                        fn from(value: &str) -> Self {
                            match value {
                                #(#from_trait),*,
                                _ => panic!("Can not create '{}' from '{}'", stringify!(#ident), value)
                            }
                        }
                    }
                }.into();
            } else {
                return quote!{
                    impl #ident {
                        #vispub fn variants() -> ::std::vec::Vec<&'static str> {
                            let mut v = vec![];
                            #(v.push(#variants));*;
                            v
                        }
                    }
                }.into()
            }
        },
        syn::Data::Union(_) => panic!("Union is not supported. Only struct allowed!"),
    }
}

