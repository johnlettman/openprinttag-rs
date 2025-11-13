pub mod util;

use quote2::ToTokens;
use syn::{parse_quote, Attribute, Field, Type, Variant, Visibility, __private::quote::format_ident, punctuated::Punctuated, token::Comma, File};
use crate::schema::name;

pub trait GetSchemaName {
    fn get_schema_name(&self) -> String;
}

pub trait GetIdent {
    #[inline]
    fn makes_ident(&self) -> bool {
        self.get_name().is_some()
    }

    fn get_name(&self) -> Option<String>;

    fn get_ident(&self) -> Option<syn::Ident> {
        Some(format_ident!("{}", self.get_name()?))
    }
}

impl<S> GetIdent for S
where
    S: GetSchemaName,
{
    #[inline]
    fn makes_ident(&self) -> bool {
        !self.get_schema_name().is_empty()
    }

    fn get_name(&self) -> Option<String> {
        self.makes_ident().then_some(name::to_camel(self.get_schema_name()))
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
    #[inline]
    fn makes_doc(&self) -> bool {
        self.get_doc().is_some()
    }

    fn get_doc(&self) -> Option<String>;

    fn get_doc_attribute(&self) -> Option<Attribute> {
        let doc = self.get_doc()?;
        Some(parse_quote!(#[doc = #doc]))
    }
}

pub trait GetDocAsAttributes: GetDoc {}

pub trait GetAttributes {
    #[inline]
    fn makes_attributes(&self) -> bool {
        !self.get_attributes().is_empty()
    }

    fn get_attributes(&self) -> Vec<Attribute>;
}

impl<I> GetAttributes for I
where
    I: GetDocAsAttributes,
{
    #[inline(always)]
    fn makes_attributes(&self) -> bool {
        self.makes_doc()
    }

    fn get_attributes(&self) -> Vec<Attribute> {
        self.get_doc_attribute().map(|d| vec![d]).unwrap_or_default()
    }
}

pub trait GetVariant {
    #[inline]
    fn makes_variant(&self) -> bool {
        self.get_variant().is_some()
    }

    fn get_variant(&self) -> Option<Variant>;
}

pub trait GetVariants {
    #[inline]
    fn makes_variants(&self) -> bool {
        !self.get_variants().is_empty()
    }

    fn get_variants(&self) -> Vec<Variant>;

    #[inline]
    fn get_variants_punctuated(&self) -> Punctuated<Variant, Comma> {
        Punctuated::from_iter(self.get_variants())
    }
}

impl<V: GetVariant> GetVariants for V {
    #[inline(always)]
    fn makes_variants(&self) -> bool {
        self.makes_variant()
    }

    #[inline]
    fn get_variants(&self) -> Vec<Variant> {
        self.get_variant().map(|v| vec![v]).unwrap_or_default()
    }
}

pub trait GetType {
    #[inline]
    fn makes_type(&self) -> bool {
        self.get_type().is_some()
    }

    fn get_type(&self) -> Option<Type>;
}

pub trait GetField {
    #[inline]
    fn makes_field(&self) -> bool {
        self.get_field().is_some()
    }

    fn get_field(&self) -> Option<Field>;
}

pub trait GetFields {
    #[inline]
    fn makes_fields(&self) -> bool {
        !self.get_fields().is_empty()
    }

    fn get_fields(&self) -> Vec<Field>;

    #[inline]
    fn get_fields_punctuated(&self) -> Punctuated<Field, Comma> {
        Punctuated::from_iter(self.get_fields())
    }
}

impl<F: GetField> GetFields for F {
    #[inline(always)]
    fn makes_fields(&self) -> bool {
        self.makes_field()
    }

    #[inline]
    fn get_fields(&self) -> Vec<Field> {
        self.get_field().map(|f| vec![f]).unwrap_or_default()
    }
}

pub trait AsItem {
    #[inline]
    fn is_item(&self) -> bool {
        self.as_item().is_some()
    }

    fn as_item(&self) -> Option<syn::Item>;
}

pub trait AsItems {
    #[inline]
    fn makes_items(&self) -> bool {
        !self.as_items().is_empty()
    }

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
