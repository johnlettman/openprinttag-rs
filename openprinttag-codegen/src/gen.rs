use proc_macro2::{Ident, Span};
use syn::{
    AttrStyle, Attribute, Expr, ExprLit, Lit, LitInt, LitStr, Meta, MetaNameValue, Type, TypeArray,
    TypePath, Visibility,
};

/// Creates a new [`Ident`] from a string, using [`Span::call_site`] as its span.
pub(crate) fn make_ident<S: AsRef<str>>(name: S) -> Ident {
    Ident::new(name.as_ref(), Span::call_site())
}

/// Parses a string into a [`Type`] node.
///
/// # Errors
/// - [`crate::Error::GenTypeError`] if the provided string cannot be parsed
/// as a valid Rust type path.
pub(crate) fn make_type<S: AsRef<str>>(path: S) -> crate::Result<Type> {
    Ok(Type::Path(TypePath {
        qself: None,
        path: syn::parse_str(path.as_ref()).map_err(crate::Error::GenTypeError)?,
    }))
}

/// Returns a public [`Visibility`] marker (`pub`).
///
/// Used to mark generated types, structs, or enums as public within generated code.
#[inline]
pub(crate) fn make_pub_visibility() -> Visibility {
    Visibility::Public(Default::default())
}

/// Constructs a fixed-size array type node [`TypeArray`] (e.g. `[u8; 32]`).
///
/// # Arguments
/// - `elem`: Element type (e.g. `"u8"` or `"MyType"`).
/// - `len`: Array length.
///
/// # Errors
/// - [`crate::Error::GenTypeError`] if the element type cannot be parsed.
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

/// Constructs a `#[doc = "..."]` [`Attribute`] for generated items.
///
/// Used to attach documentation comments to generated enums, variants, or fields
/// at the syntax tree level.
///
/// # Errors
/// - [`crate::Error::GenError`] if the internal `doc` path cannot be parsed.
///
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
