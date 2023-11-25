use syn::DeriveInput;


pub fn extract_doc(attrs: &Vec<syn::Attribute>) -> Option<String> {
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

pub fn extract_attr(attrs: &Vec<syn::Attribute>) -> Option<(String, String)> {
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

pub fn root_extract(input: DeriveInput) -> (Vec<syn::Attribute>, syn::Visibility, syn::Ident, syn::Data)  {
    let DeriveInput { attrs, vis, ident, data, .. } = input;
    (attrs, vis, ident, data) 
}

pub fn extract_fields(data: syn::Data) -> Option<syn::Fields> {
    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = data {
        return Some(fields)
    }
    None
}

pub fn extract_fields_names(fields: syn::Fields) -> Option<syn::punctuated::Punctuated<syn::Field, syn::token::Comma>> {
    let syn::Fields::Named(syn::FieldsNamed{ named, .. }) = fields else { return None };
    Some(named)
}

