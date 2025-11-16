pub mod util;

use crate::schema::name;
use quote2::format_ident;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, Attribute, Field, File, Type, Variant,
    Visibility,
};

pub trait GetSchemaName {
    fn get_schema_name(&self) -> String;
}

pub trait GetIdent {
    fn get_name(&self) -> Option<String>;

    #[inline]
    fn get_core_ident(&self) -> Option<syn::Ident> {
        Some(format_ident!("{}", self.get_name()?))
    }
}

impl<S> GetIdent for S
where
    S: GetSchemaName,
{
    #[inline]
    fn get_name(&self) -> Option<String> {
        Some(name::to_camel(self.get_schema_name()))
    }
}

pub trait GetVisibility {
    fn get_visibility(&self) -> Visibility;
}

pub trait GetPubVisibility {}

impl<I> GetVisibility for I
where
    I: GetPubVisibility,
{
    #[inline(always)]
    fn get_visibility(&self) -> Visibility {
        parse_quote!(pub)
    }
}

pub trait GetDoc {
    fn get_doc(&self) -> Option<String>;

    fn get_doc_attribute(&self) -> Option<Attribute> {
        let doc = self.get_doc()?;
        Some(parse_quote!(#[doc = #doc]))
    }
}

pub trait GetDocAsAttributes: GetDoc {}

pub trait GetAttributes {
    fn get_core_attributes(&self) -> Vec<Attribute>;
}

impl<I> GetAttributes for I
where
    I: GetDocAsAttributes,
{
    fn get_core_attributes(&self) -> Vec<Attribute> {
        self.get_doc_attribute().map(|d| vec![d]).unwrap_or_default()
    }
}

pub trait GetVariant {
    fn get_core_variant(&self) -> Option<Variant>;
}

pub trait GetVariants {
    fn get_core_variants(&self) -> Vec<Variant>;

    #[inline]
    fn get_core_variants_punctuated(&self) -> Punctuated<Variant, Comma> {
        Punctuated::from_iter(self.get_core_variants())
    }
}

impl<V: GetVariant> GetVariants for V {
    #[inline]
    fn get_core_variants(&self) -> Vec<Variant> {
        self.get_core_variant().map(|v| vec![v]).unwrap_or_default()
    }
}

pub trait GetType {
    fn get_core_type(&self) -> Option<Type>;
}

pub trait GetField {
    fn get_core_field(&self) -> Option<Field>;
}

pub trait GetSize {
    fn get_size(&self) -> usize;
}

pub trait GetFields {
    fn get_core_fields(&self) -> Vec<Field>;

    #[inline]
    fn get_fields_punctuated(&self) -> Punctuated<Field, Comma> {
        Punctuated::from_iter(self.get_core_fields())
    }
}

impl<F: GetField> GetFields for F {
    #[inline]
    fn get_core_fields(&self) -> Vec<Field> {
        self.get_core_field().map(|f| vec![f]).unwrap_or_default()
    }
}

pub trait ToItem {
    fn to_core_item(&self) -> Option<syn::Item>;
}

pub trait ToItems {
    fn to_core_items(&self) -> Vec<syn::Item>;
}

impl<I> ToItems for I
where
    I: ToItem,
{
    #[inline]
    fn to_core_items(&self) -> Vec<syn::Item> {
        self.to_core_item().map(|i| vec![i]).unwrap_or_default()
    }
}

pub trait ToFile {
    fn to_core_file(&self) -> File;

    fn to_core_file_string(&self) -> String {
        let file = self.to_core_file();

        #[cfg(feature = "format")]
        return prettyplease::unparse(&file);

        #[cfg(not(feature = "format"))]
        return file.to_token_stream().to_string();
    }
}

impl<I> ToFile for I
where
    I: ToItems + GetAttributes,
{
    #[inline(always)]
    fn to_core_file(&self) -> File {
        File { shebang: None, attrs: self.get_core_attributes(), items: self.to_core_items() }
    }
}
