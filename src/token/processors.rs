use quote::quote;
use super::{extractor::extract_attr, extract_doc};

type Punc = syn::punctuated::Punctuated<syn::Field, syn::token::Comma>;
type TokenStream2 = proc_macro2::TokenStream;

pub fn get_vis_pub() -> syn::Visibility {
    let pub_token = syn::token::Pub::default();
    syn::Visibility::Public(pub_token)
}

/// process TokenStream for args to be use with new() method
pub fn new_args(punc: &Punc) -> impl Iterator<Item = TokenStream2> + '_ {
    let new_args = punc.iter().map(|f| {
        if let Some(_) = &f.ident {
            let id = &f.ident;
            let ty = &f.ty;
            quote!{ #id : #ty }
        } else { quote!() }
    });
    new_args
}

pub fn set_val(punc: &Punc) -> impl Iterator<Item = TokenStream2> + '_ {
    punc.iter().map(|f| {
        if let Some(id) = &f.ident {
            quote!( #id )
        } else { quote!() }
    })
}


/// process TokenStream for tuple arg to be use with fields method
pub fn tup_args(punc: &Punc) -> impl Iterator<Item = TokenStream2> + '_ {
    punc.iter().map(|f| {
        if let Some(_id) = &f.ident {
            let id = &f.ident;
            let ty = &f.ty;
            quote!{ (stringify!(#id), stringify!(#ty)) }
        } else { quote!() }
    })
}

pub fn set_methods(punc: &Punc, vis: syn::Visibility) -> impl Iterator<Item = TokenStream2> + '_ {
    punc.iter().map(move |f| {
        if let Some(id) = &f.ident {
            let x = extract_attr(&f.attrs);
            if let Some((name, value)) = x {
                if name.as_str() == "exclude" || (name.as_str() == "only" && value.as_str() != "set") {
                    return quote!()
                } 
            } 
            let methodname = syn::Ident::new(&format!("set_{}", id.to_string()), id.span());
            let ty = &f.ty;
            quote!{
                #vis fn #methodname(&mut self, value: #ty) {
                    self.#id = value
                }
            }
        } else { quote!() }
    })
}

pub fn get_methods(punc: &Punc, vis: syn::Visibility) -> impl Iterator<Item = TokenStream2> + '_ {
    punc.iter().map(move |f| {
        if let Some(id) = &f.ident {
            let x = extract_attr(&f.attrs);
            if let Some((name, value)) = x {
                if name.as_str() == "exclude" || (name.as_str() == "only" && value.as_str() != "get") {
                    return quote!()
                } 
            }
            let methodname = syn::Ident::new(&format!("get_{}", id.to_string()), id.span());
            let ty = &f.ty;
            quote!{
                #vis fn #methodname(&self) -> &#ty {
                    &self.#id
                }
            }
        } else { quote!() }
    })
}

pub fn info(attrs: &Vec<syn::Attribute>, vis: syn::Visibility) -> TokenStream2 {
    if let Some(doctxt) = extract_doc(attrs) {
        quote!{
            /// Get struct info as defined on it's documentation
            #vis fn info() -> ::std::string::String {
                ::std::string::String::from(#doctxt)
            }
        }
    } else { quote!() }
}

