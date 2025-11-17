use syn::{parse_quote, parse_str, Attribute, Type};

/// Parses a string into a [`Type`] node.
///
/// # Errors
/// - [`crate::Error::SynError`] if the provided string cannot be parsed
///   as a valid Rust type path.
#[inline]
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", fields(?path = path.as_ref()), skip(path)))]
pub fn emit_type<S: AsRef<str>>(path: S) -> Type {
    parse_str::<Type>(path.as_ref()).expect("failed to parse type")
}

#[inline]
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip(docs)))]
pub fn emit_doc_attribute<D: AsRef<str> + quote2::ToTokens>(docs: D) -> Attribute {
    parse_quote!(#[doc = #docs])
}
