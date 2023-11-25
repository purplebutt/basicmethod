#![allow(unused)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

fn extract_doc(attrs: &Vec<syn::Attribute>) -> Option<String> {
    if let Some(att) = attrs.first() {
        let syn::Meta::NameValue(ref metval) = att.meta else { return None };
        let syn::Expr::Lit(ref exprlit) = metval.value else { return None };
        let syn::Lit::Str(ref litstr) = exprlit.lit else { return None };
        let mut s = litstr.token().to_string();
        s.remove(0);
        s.pop();
        Some(s.trim().to_string())
    } else { None }
}

fn extract_attr(attrs: &Vec<syn::Attribute>) -> Option<(String, String)> {
    if let Some(att) = attrs.first() {
        let name; let value;

        match att.meta {
            syn::Meta::Path(ref metpath) => {
                let Some(ident) = metpath.get_ident() else { return None };
                Some((ident.to_string(), "".to_string()))
            }
            syn::Meta::NameValue(ref metval) => {
                let attpath = &metval.path.segments;
                name = match attpath.first() {
                    Some(ap) => {
                        ap.ident.to_string()
                    }
                    None => return None
                };

                let syn::Expr::Lit(ref exprlit) = metval.value else { return None };
                let syn::Lit::Str(ref litstr) = exprlit.lit else { return None };
                let mut s = litstr.token().to_string();
                s.remove(0);
                s.pop();
                value = s.trim().to_string();
                Some((name, value))
            }
            _ => return None
        }
    } else { None }
}

#[proc_macro_derive(BasicMethod, attributes(only,exclude))]
pub fn basic_method_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let DeriveInput { ident, attrs, data, .. } = ast;
    let doc = extract_doc(&attrs);
    let struct_name = &ident.to_string();
    println!("{:#?}", data);
    if let syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fname), .. }) = data {
        let syn::FieldsNamed{ named, .. } = fname;
        
        let new_args = named.iter().map(|f| {
            if let Some(id) = &f.ident {
                let id = &f.ident;
                let ty = &f.ty;
                quote!{ #id : #ty }
            } else { quote!() }
        });
        let new_setval = named.iter().map(|f| {
            if let Some(id) = &f.ident {
                quote!( #id )
            } else { quote!() }
        });
        let fields = named.iter().map(|f| {
            if let Some(ref fname) = f.ident {
                let name = fname.to_string();
                quote!{ #name }
            } else { quote!() }
        });
        let set_methods = named.iter().map(|f| {
            if let Some(id) = &f.ident {
                let x = extract_attr(&f.attrs);
                if let Some((name, value)) = x {
                    if name.as_str() == "exclude" || (name.as_str() == "only" && value.as_str() != "set") {
                        return quote!()
                    } else {
                        //NOTE: duplicate code
                        let methodname = syn::Ident::new(&format!("set_{}", id.to_string()), id.span());
                        let ty = &f.ty;
                        return quote!{
                            pub fn #methodname(&mut self, value: #ty) {
                                self.#id = value
                            }
                        }
                    }
                } else {
                    //NOTE: duplicate code
                    let methodname = syn::Ident::new(&format!("set_{}", id.to_string()), id.span());
                    let ty = &f.ty;
                    quote!{
                        pub fn #methodname(&mut self, value: #ty) {
                            self.#id = value
                        }
                    }
                }
            } else { quote!() }
        });
        let get_methods = named.iter().map(|f| {
            if let Some(id) = &f.ident {
                let x = extract_attr(&f.attrs);
                if let Some((name, value)) = x {
                    if name.as_str() == "exclude" || (name.as_str() == "only" && value.as_str() != "get") {
                        quote!()
                    } else {
                        //NOTE: duplicate code
                        let methodname = syn::Ident::new(&format!("get_{}", id.to_string()), id.span());
                        let ty = &f.ty;
                        quote!{
                            pub fn #methodname(&self) -> &#ty {
                                &self.#id
                            }
                        }
                    }
                } else {
                    //NOTE: duplicate code
                    let methodname = syn::Ident::new(&format!("get_{}", id.to_string()), id.span());
                    let ty = &f.ty;
                    quote!{
                        pub fn #methodname(&self) -> &#ty {
                            &self.#id
                        }
                    }
                }
            } else { quote!() }
        });
        let info = {
            if let Some(doctxt) = doc {
                quote!{
                    /// Get struct info as defined on it's documentation
                    pub fn info() -> ::std::string::String {
                        ::std::string::String::from(#doctxt)
                    }
                }
            } else { quote!() }
        };
    
        return quote!{
            impl #ident {
                pub fn new(#(#new_args),*) -> Self {
                    Self { #(#new_setval),* }
                }
                #info
                pub fn fields() -> ::std::vec::Vec<(&'static str, &'static str)> {
                    let mut v = vec![];
                    #(v.push((#fields, stringify!(#fields))));*;
                    v
                }
                #(#set_methods)*
                #(#get_methods)*
            }
        }.into()
    }
    panic!("'{}' is not supported!", ident.to_string())
}

