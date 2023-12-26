#![allow(dead_code)]
use quote::quote;

/// Check if type is number (integer, float, isize, or usize)
pub fn is_number(ty: &str) -> bool {
    match ty {
        "i8" | "i16" | "i32" | "i64" | "i128" => true,
        "u8" | "u16" | "u32" | "u64" | "u128" => true,
        "f32" | "f64" => true,
        "isize" | "usize" => true,
        _ => false
    }
}

/// Check if type is unsigned integer, return true if yes
pub fn is_unsigned(ty: &str) -> bool {
    match ty {
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => true,
        _ => false
    }
}

/// Check if type is signed integer, return true if yes
pub fn is_signed(ty: &str) -> bool {
    match ty {
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => true,
        _ => false
    }
}

/// Check if type is float, return true if yes
pub fn is_float(ty: &str) -> bool {
    match ty {
        "f32" | "f64" => true,
        _ => false
    }
}

pub fn inc_dec(fname: &syn::Ident, ftype: &syn::Type, vis: &syn::Visibility) -> proc_macro2::TokenStream {
    let increase = syn::Ident::new(&format!("inc_{}", fname.to_string()), fname.span());
    let decrease = syn::Ident::new(&format!("dec_{}", fname.to_string()), fname.span());
    let dec_clamped = syn::Ident::new(&format!("dec_tozero_{}", fname.to_string()), fname.span());
    quote!{
        /// Update the old value and returning the newly updated value
        /// If the old value is 5, and by is 3, it'll be updated to 8.
        #vis fn #increase(&mut self, by: #ftype) -> #ftype {
            self.#fname += by;
            self.#fname
        }
        /// Update the old value and returning the newly updated value
        /// If the old value is 5, and by is 3, it'll be updated to 2.
        #vis fn #decrease(&mut self, by: #ftype) -> #ftype {
            self.#fname -= by;
            self.#fname
        }
        /// This method is usefull for unsigned integer type.
        /// Update the old value and returning the newly updated value
        /// This method will not panic even if by greater than the old value
        /// If the old value is 5, and by is 6, instead of panic (let assume type is unsigned)
        /// old value will be updated to zero (0).
        #vis fn #dec_clamped(&mut self, by: #ftype) -> #ftype {
            self.#fname -= by.clamp(0 as #ftype, self.#fname);
            self.#fname
        }
    }
}

pub fn inc_dec_unnamed(idx: &syn::Index, ftype: &syn::Ident, vis: &syn::Visibility) -> proc_macro2::TokenStream {
    let increase = syn::Ident::new(&format!("inc_{}_{}", ftype.to_string(), idx.index), ftype.span());
    let decrease = syn::Ident::new(&format!("dec_{}_{}", ftype.to_string(), idx.index), ftype.span());
    let dec_clamp = syn::Ident::new(&format!("dec_tozero_{}_{}", ftype.to_string(), idx.index), ftype.span());
    quote!{
        /// Update the old value and returning the newly updated value
        /// If the old value is 5, and by is 3, it'll be updated to 8.
        #vis fn #increase(&mut self, by: #ftype) -> #ftype {
            self.#idx += by;
            self.#idx
        }
        /// Update the old value and returning the newly updated value
        /// If the old value is 5, and by is 3, it'll be updated to 2.
        #vis fn #decrease(&mut self, by: #ftype) -> #ftype {
            self.#idx -= by;
            self.#idx
        }
        /// This method is usefull for unsigned integer type.
        /// Update the old value and returning the newly updated value
        /// This method will not panic even if by greater than the old value
        /// If the old value is 5, and by is 6, instead of panic (let assume type is unsigned)
        /// old value will be updated to zero (0).
        #vis fn #dec_clamp(&mut self, by: #ftype) -> #ftype {
            self.#idx -= by.clamp(0 as #ftype, self.#idx);
            self.#idx
        }
    }
}

