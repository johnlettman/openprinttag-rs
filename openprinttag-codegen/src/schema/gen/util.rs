use syn::{parse_str, Type};

/// Parses a string into a [`Type`] node.
///
/// # Errors
/// - [`crate::Error::GenTypeError`] if the provided string cannot be parsed
/// as a valid Rust type path.
#[inline]
pub fn make_type<S: AsRef<str>>(path: S) -> crate::Result<Type> {
    parse_str::<Type>(path.as_ref()).map_err(crate::Error::SynError)
}
