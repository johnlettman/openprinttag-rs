use proc_macro2::{Ident, Span};
use syn::{
    AttrStyle, Attribute, Expr, ExprLit, Lit, LitInt, LitStr, Meta, MetaNameValue, Type, TypeArray,
    TypePath, Visibility,
};

pub(crate) fn make_ident<S: AsRef<str>>(name: S) -> Ident {
    Ident::new(name.as_ref(), Span::call_site())
}

pub(crate) fn make_type<S: AsRef<str>>(path: S) -> crate::Result<Type> {
    Ok(Type::Path(TypePath {
        qself: None,
        path: syn::parse_str(path.as_ref()).map_err(crate::Error::GenTypeError)?,
    }))
}

#[inline]
pub(crate) fn make_pub_visibility() -> Visibility {
    Visibility::Public(Default::default())
}

pub(crate) fn make_array_type<E: AsRef<str>>(elem: E, len: usize) -> crate::Result<Type> {
    Ok(Type::Array(TypeArray {
        bracket_token: Default::default(),
        elem: Box::new(make_type(elem)?),
        semi_token: Default::default(),
        len: Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: Lit::Int(LitInt::new(&len.to_string(), Span::call_site())),
        }),
    }))
}

pub(crate) fn make_doc_attribute<S: AsRef<str>>(doc: S) -> crate::Result<Attribute> {
    Ok(Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::NameValue(MetaNameValue {
            path: syn::parse_str("doc")?,
            eq_token: Default::default(),
            value: Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Str(LitStr::new(doc.as_ref(), Span::call_site())),
            }),
        }),
    })
}
