pub mod util;

use quote2::format_ident;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, Attribute, Field, File, Type, Variant,
    Visibility,
};
use crate::schema::{name, HasDocs, SchemaName};

pub trait EmitIdent {
    fn get_name(&self) -> Option<String>;

    #[inline]
    fn rs_core_ident(&self) -> Option<syn::Ident> {
        Some(format_ident!("{}", self.get_name()?))
    }
}

impl<S> EmitIdent for S
where
    S: SchemaName,
{
    #[inline]
    fn get_name(&self) -> Option<String> {
        Some(name::to_camel(self.get_schema_name()))
    }
}

/// Produces a [`Visibility`] for a generated Rust item.
///
/// This trait is used during code generation to determine whether a generated
/// struct, enum, field, or function should be `pub`, `pub(crate)`, or private.
/// Implementors provide the visibility appropriate for a schema element.
///
/// A common implementation, [`EmitPubVisibility`], simply returns `pub` for all
/// generated items.
pub trait EmitVisibility {
    /// Returns the [`Visibility`] to use for the generated Rust item.
    fn emit_visibility(&self) -> Visibility;
}

/// Produces the Rust [`Type`] associated with a schema node.
///
/// This trait is implemented by schema elements that correspond to typed fields
/// or values (e.g., integer, string, enum, boolean...). The returned [`Type`]
/// determines the actual Rust representation used in the generated code.
///
/// Returning [`None`] indicates that the schema element does not directly map
/// to a type (e.g., a placeholder or empty node), and should therefore be
/// skipped.
pub trait EmitType {
    /// Generates the Rust [`Type`] representing this schema element for the
    /// `openprinttag-core` crate.
    fn emit_core_type(&self) -> Option<Type>;
}

pub trait EmitPubVisibility {}

impl<I> EmitVisibility for I
where
    I: EmitPubVisibility,
{
    #[inline(always)]
    fn emit_visibility(&self) -> Visibility {
        parse_quote!(pub)
    }
}

/// Produces the [`Vec`] of [`Attribute`] that should be attached to a generated
/// Rust item.
///
/// Attributes include things like:
/// - documentation comments (`#[doc = "..."]`)
/// - derives (`#[derive(...)]`)
/// - custom markers
///
/// This trait allows schema elements to supply code-level annotations
/// that reflect schema metadata. The returning list is inserted directly into
/// a [`Item`], [`Field`], [`Variant`], or similar structure.
pub trait EmitAttributes {
    /// Returns the [`Vec`] of [`Attribute`] to attach to the generated Rust
    /// item for the `openprinttag-core` crate.
    fn emit_core_attributes(&self) -> Vec<Attribute>;
}

pub trait EmitDocsAsAttributes: HasDocs {}

impl<I> EmitAttributes for I where I: EmitDocsAsAttributes {
    fn emit_core_attributes(&self) -> Vec<Attribute> {
        if let Some(docs) = self.docs() {
            vec![parse_quote!(#[doc = #docs])]
        } else {
            Vec::new()
        }
    }
}

/// Produces a single [`Variant`] for a generated enum.
///
/// This trait corresponds to a schema element that describes a single variant
/// within an enumeration. Implementors control:
/// - variant name
/// - variant attributes ([`EmitAttributes`])
/// - variant fields (if any)
///
/// If the schema variant does not map to a Rust variant (for example,
/// deprecated or optional nodes), the implementor may return [`None`].
pub trait EmitVariant {
    /// Returns the constructed enum variant for the `openprinttag-core` crate,
    /// or [`None`] if it should be omitted.
    fn emit_core_variant(&self) -> Option<Variant>;
}

/// Produces all enum variants associated with a schema element.
///
/// This trait is used by schema elements that define an entire enumeration.
/// It is the multi-value counterpart to [`EmitVariant`].
pub trait EmitVariants {
    /// Returns all enum variants to be included in the generated enum for the
    /// `openprinttag-core` crate.
    fn emit_core_variants(&self) -> Vec<Variant>;

    #[inline]
    fn get_core_variants_punctuated(&self) -> Punctuated<Variant, Comma> {
        Punctuated::from_iter(self.emit_core_variants())
    }
}

impl<V: EmitVariant> EmitVariants for V {
    #[inline]
    fn emit_core_variants(&self) -> Vec<Variant> {
        self.emit_core_variant().map(|v| vec![v]).unwrap_or_default()
    }
}

/// Produces a fully constructed [`Field`] for a generated Rust struct.
///
/// This trait encapsulates the logic for converting a schema-defined field
/// into an actual Rust struct field, including:
/// - field name
/// - field type ([`EmitType`])
/// - field attributes ([`EmitAttributes`])
/// - visibility ([`EmitVisibility`])
///
/// If the schema field does not map to a Rust field (for example, deprecated
/// or optional nodes), the implementor may return [`None`].
pub trait EmitField {
    /// Returns the constructed struct field for the `openprinttag-core` crate,
    /// or [`None`] if it should be omitted.
    fn emit_core_field(&self) -> Option<Field>;
}

/// Produces all [`Field`] values associated with a schema element.
///
/// This trait is implemented by types that contain multiple schema-defined
/// fields, such as objects, structs, or generated record types. It is the
/// multi-value counterpart to [`EmitField`].
///
/// Implementors typically aggregate many [`EmitField`] entries into a vector.
pub trait EmitFields {
    /// Returns all struct fields that should appear in the generated type for
    /// the `openprinttag-core` crate.
    fn emit_core_fields(&self) -> Vec<Field>;
}

impl<F: EmitField> EmitFields for F {
    #[inline]
    fn emit_core_fields(&self) -> Vec<Field> {
        self.emit_core_field().map(|f| vec![f]).unwrap_or_default()
    }
}

/// Produces a single top-level Rust item [`Item`] from a schema element.
///
/// This trait is implemented by schema nodes that directly generate structs,
/// enums, type aliases, functions, or other Rust items. A single entry in a
/// schema may or may not correspond to a Rust item; a returned [`None`] means
/// that no top-level item should be emitted.
pub trait EmitItem {
    /// Returns the generated item for the `openprinttag-core` crate, or
    /// [`None`] if nothing should be emitted.
    fn emit_core_item(&self) -> Option<syn::Item>;
}

/// Produces a list of [`Item`] values for a schema element.
///
/// This trait is the multi-value counterpart to [`EmitItem`], used by schema
/// roots or aggregating structures that produce more than one Rust item.
///
/// Default implementations typically wrap a single item and return [`Vec`] of
/// [`Item`].
pub trait EmitItems {
    fn emit_core_items(&self) -> Vec<syn::Item>;
}

impl<I> EmitItems for I
where
    I: EmitItem,
{
    #[inline]
    fn emit_core_items(&self) -> Vec<syn::Item> {
        self.emit_core_item().map(|i| vec![i]).unwrap_or_default()
    }
}

/// Produces a complete [`File`] representing the output Rust source file.
///
/// This trait is implemented by high-level schema elements that can render
/// a full compilation unit.
pub trait EmitFile {
    /// Returns the generated Rust [`File`] for the `openprinttag-core` crate.
    fn emit_core_file(&self) -> File;

    /// Returns the rendered file contents as a string for the
    /// `openprinttag-core` crate.
    ///
    /// - if the `"format"` feature is enabled, this uses `prettyplease`.
    /// - otherwise it emits raw tokens.
    fn emit_core_file_string(&self) -> String {
        let file = self.emit_core_file();

        #[cfg(feature = "format")]
        return prettyplease::unparse(&file);

        #[cfg(not(feature = "format"))]
        return file.to_token_stream().to_string();
    }
}

impl<I> EmitFile for I
where
    I: EmitItems + EmitAttributes,
{
    #[inline(always)]
    fn emit_core_file(&self) -> File {
        File { shebang: None, attrs: self.emit_core_attributes(), items: self.emit_core_items() }
    }
}
