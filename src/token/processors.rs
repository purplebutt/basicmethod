use quote::quote;
use crate::token::extract_type_name;

use super::{extractor::extract_attr, extract_doc, helper::{is_number, inc_dec, inc_dec_unnamed}};

type Punc = syn::punctuated::Punctuated<syn::Field, syn::token::Comma>;
type Punc2 = syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>;
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

pub fn get_methods_mut(punc: &Punc, vis: syn::Visibility) -> impl Iterator<Item = TokenStream2> + '_ {
    punc.iter().map(move |f| {
        if let Some(id) = &f.ident {
            let x = extract_attr(&f.attrs);
            if let Some((name, value)) = x {
                if name.as_str() == "exclude" || (name.as_str() == "only" && value.as_str() != "get") {
                    return quote!()
                } 
            }
            let methodname = syn::Ident::new(&format!("get_{}_mut", id.to_string()), id.span());
            let ty = &f.ty;
            quote!{
                #vis fn #methodname(&mut self) -> &mut #ty {
                    &mut self.#id
                }
            }
        } else { quote!() }
    })
}

// if number add increment, decrement and dec_tozero method
pub fn incdec_methods(punc: &Punc, vis: syn::Visibility) -> impl Iterator<Item = TokenStream2> + '_ {
    punc.iter().map(move |f| {
        if let Some(id) = &f.ident {
            let ty = &f.ty;
            let type_str = extract_type_name(ty.clone());
            if is_number(&type_str) {
                let x = extract_attr(&f.attrs);
                if let Some((name, value)) = x {
                    if name.as_str() == "exclude" || (name.as_str() == "only" && value.as_str() != "inc") {
                        return quote!()
                    } 
                }
                inc_dec(id, ty, &vis)
                // let methodname = syn::Ident::new(&format!("inc_{}", id.to_string()), id.span());
                // quote!{
                //     /// Update the old value with old value + value, returning the newly updated
                //     /// value
                //     /// If the old value is 5, and value is 3, it'll be updated to 8.
                //     #vis fn #methodname(&mut self, value: #ty) -> #ty {
                //         self.#id += value;
                //         self.#id
                //     }
                // }
            } else { quote!() }
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

// unnamed or tuple struct
pub fn unamed_field(punc: &Punc, _vis: syn::Visibility) -> impl Iterator<Item = TokenStream2> + '_ {
    let mut idx = 0;
    punc.iter().map(move |f| {
        match &f.ty {
            syn::Type::Path(typepath) => {
                let sgmnts = &typepath.path.segments[0];
                let ty = &sgmnts.ident;
                let newident = syn::Ident::new(&format!("{}_{}", ty.to_string(), idx), ty.span());
                idx += 1;
                quote!(stringify!(#newident))
            }
            _ => quote!()
        } 
    })
}

pub fn set_methods_unnamed(punc: &Punc, vis: syn::Visibility) -> impl Iterator<Item = TokenStream2> + '_ {
    let mut idx = syn::Index::from(0);
    punc.iter().map(move |f| {
        let x = extract_attr(&f.attrs);
        if let Some((name, value)) = x {
            if name.as_str() == "exclude" || (name.as_str() == "only" && value.as_str() != "set") {
                return quote!()
            } 
        }
        match &f.ty {
            syn::Type::Path(typepath) => {
                let sgmnts = &typepath.path.segments[0];
                let ty = &sgmnts.ident;
                let methodname = syn::Ident::new(&format!("set_{}_{}", ty.to_string(), idx.index), ty.span());
                let result = quote!{
                    #[allow(non_snake_case)]
                    #vis fn #methodname(&mut self, value: #ty) {
                        self.#idx = value
                    }
                };
                idx.index += 1;
                result
            }
            _ => quote!()
        } 
    })
}

pub fn incdec_methods_unnamed(punc: &Punc, vis: syn::Visibility) -> impl Iterator<Item = TokenStream2> + '_ {
    let mut idx = 0;
    punc.iter().map(move |f| {
        idx += 1;
        let type_str = extract_type_name(f.ty.clone());
        if is_number(&type_str) {
            let x = extract_attr(&f.attrs);
            if let Some((name, value)) = x {
                if name.as_str() == "exclude" || (name.as_str() == "only" && value.as_str() != "inc") {
                    return quote!()
                } 
            }
            match &f.ty {
                syn::Type::Path(typepath) => {
                    let idx = syn::Index::from(idx-1);
                    let sgmnts = &typepath.path.segments[0];
                    let ty = &sgmnts.ident;
                    inc_dec_unnamed(&idx, ty, &vis)
                    // let methodname = syn::Ident::new(&format!("inc_{}_{}", ty.to_string(), idx.index), ty.span());
                    // let result = quote!{
                    //     #[allow(non_snake_case)]
                    //     /// Update the old value with old value + value, returning the newly updated
                    //     /// value
                    //     /// If the old value is 5, and value is 3, it'll be updated to 8.
                    //     #vis fn #methodname(&mut self, value: #ty) -> #ty {
                    //         self.#idx += value;
                    //         self.#idx
                    //     }
                    // };
                    // result
                }
                _ => quote!()
            } 
        } else { quote!() }
    })
}

pub fn get_methods_unnamed(punc: &Punc, vis: syn::Visibility) -> impl Iterator<Item = TokenStream2> + '_ {
    let mut idx = syn::Index::from(0);
    punc.iter().map(move |f| {
        let x = extract_attr(&f.attrs);
        if let Some((name, value)) = x {
            if name.as_str() == "exclude" || (name.as_str() == "only" && value.as_str() != "get") {
                return quote!()
            } 
        }
        match &f.ty {
            syn::Type::Path(typepath) => {
                let sgmnts = &typepath.path.segments[0];
                let ty = &sgmnts.ident;
                let methodname = syn::Ident::new(&format!("get_{}_{}", ty.to_string(), idx.index), ty.span());
                let result = quote!{
                    #[allow(non_snake_case)]
                    #vis fn #methodname(&self) -> &#ty {
                        &self.#idx
                    }
                };
                idx.index += 1;
                result
            }
            _ => quote!()
        } 
    })
}

pub fn is_enum_field_unit(variants: &Punc2) -> bool {
    for f in variants {
        let syn::Fields::Unit = f.fields else {
            return false
        };
    }
    true
}

pub fn enum_variants(variants: &Punc2) -> impl Iterator<Item = TokenStream2> + '_ {
    variants.iter().map(|f| {
        let ident = &f.ident.to_string();
        quote!(#ident)
    })
}

pub fn from_trait(variants: &Punc2) -> impl Iterator<Item = TokenStream2> + '_ {
    variants.iter().map(|f| {
        let ident = &f.ident;
        let name = &f.ident.to_string();
        quote!{
            #name => Self::#ident
        }
    })
}

