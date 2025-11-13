pub mod util;

use quote2::ToTokens;
use syn::{parse_quote, Attribute, Field, Type, Variant, Visibility, __private::quote::format_ident, punctuated::Punctuated, token::Comma, File};
use crate::schema::name;

pub trait GetSchemaName {
    fn get_schema_name(&self) -> String;
}

pub trait GetIdent {
    fn get_name(&self) -> Option<String>;

    #[inline]
    fn get_ident(&self) -> Option<syn::Ident> {
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
    fn get_attributes(&self) -> Vec<Attribute>;
}

impl<I> GetAttributes for I
where
    I: GetDocAsAttributes,
{
    fn get_attributes(&self) -> Vec<Attribute> {
        self.get_doc_attribute().map(|d| vec![d]).unwrap_or_default()
    }
}

pub trait GetVariant {
    fn get_variant(&self) -> Option<Variant>;
}

pub trait GetVariants {
    fn get_variants(&self) -> Vec<Variant>;

    #[inline]
    fn get_variants_punctuated(&self) -> Punctuated<Variant, Comma> {
        Punctuated::from_iter(self.get_variants())
    }
}

impl<V: GetVariant> GetVariants for V {
    #[inline]
    fn get_variants(&self) -> Vec<Variant> {
        self.get_variant().map(|v| vec![v]).unwrap_or_default()
    }
}

pub trait GetType {
    fn get_type(&self) -> Option<Type>;
}

pub trait GetField {
    fn get_field(&self) -> Option<Field>;
}

pub trait GetFields {
    fn get_fields(&self) -> Vec<Field>;

    #[inline]
    fn get_fields_punctuated(&self) -> Punctuated<Field, Comma> {
        Punctuated::from_iter(self.get_fields())
    }
}

impl<F: GetField> GetFields for F {
    #[inline]
    fn get_fields(&self) -> Vec<Field> {
        self.get_field().map(|f| vec![f]).unwrap_or_default()
    }
}

pub trait AsItem {
    fn as_item(&self) -> Option<syn::Item>;
}

pub trait AsItems {
    fn as_items(&self) -> Vec<syn::Item>;
}

impl<I> AsItems for I
where
    I: AsItem,
{
    #[inline]
    fn as_items(&self) -> Vec<syn::Item> {
        self.as_item().map(|i| vec![i]).unwrap_or_default()
    }
}

pub trait AsFile {
    fn as_file(&self) -> File;

    fn as_file_string(&self) -> String {
        self.as_file().to_token_stream().to_string()
    }
}

impl<I> AsFile for I where I: AsItems + GetAttributes {
    #[inline(always)]
    fn as_file(&self) -> File {
        File {
            shebang: None,
            attrs: self.get_attributes(),
            items: self.as_items()
        }
    }
}
