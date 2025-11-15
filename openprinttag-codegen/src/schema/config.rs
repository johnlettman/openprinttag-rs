use quote2::ToTokens;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use syn::{parse_quote, Attribute, Field, Fields, Item};

use crate::{
    de::DeserializeWithContext,
    loader::{CrateDirLoader, GitHubLoader},
    schema::{
        builtin,
        context::{registry::Register, Context, LocalContext, SharedContext},
        gen::{
            GetAttributes, GetDoc, GetDocAsAttributes, GetFields, GetIdent, GetPubVisibility,
            GetVisibility, ToFile, ToItems,
        },
        name,
    },
};

/// Represents an [OpenPrintTag] configuration schema.
///
/// A `Config` defines the structure of an [OpenPrintTag data layout], composed
/// of three major field groups:
///
/// - **Meta section** (`meta_fields`): Defines region offsets and sizes within
///   the NDEF payload.
/// - **Main section** (`main_fields`): Contains static material information.
/// - **Auxiliary section** (`aux_fields`): Contains dynamic runtime or
///   usage-tracking data.
///
/// This struct corresponds to the `config_*` schema files found in the crate’s
/// `data` directory. These files define the field layout, metadata, and
/// auxiliary data formats used by [OpenPrintTag]-compliant devices and
/// software.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::Config;
///
/// // load a built-in configuration such as `config_nfcv.yaml`
/// let config = Config::load("config_nfcv").expect("should load and deserialize configuration");
///
/// println!("MIME type: {}", config.mime_type);
/// println!("Main fields: {} entries", config.main_fields.len());
/// ```
///
/// # Structure
/// | Section       | Purpose                                                   | Reference           |
/// |---------------|-----------------------------------------------------------|---------------------|
/// | **Meta**      | Defines offsets, sizes, and low-level NFC layout details. | [Meta section]      |
/// | **Main**      | Defines static, per-material properties.                  | [Main section]      |
/// | **Auxiliary** | Defines dynamic or runtime tracking data.                 | [Auxiliary section] |
///
/// [Meta section]: https://specs.openprinttag.org/#/nfc_data_format?id=_4-meta-section
/// [Main section]: https://specs.openprinttag.org/#/nfc_data_format?id=_5-main-section
/// [Auxiliary section]: https://specs.openprinttag.org/#/nfc_data_format?id=_6-auxiliary-section
/// [OpenPrintTag]: https://openprinttag.org/
/// [OpenPrintTag data layout]: https://specs.openprinttag.org/#/nfc_data_format
#[derive(Debug, Clone)]
pub struct ConfigStructSchema {
    schema_name: String,

    context: SharedContext,

    /// The MIME type of this configuration (e.g.
    /// `"application/vnd.openprinttag"`).
    pub mime_type: String,

    /// The optional root section for hierarchical configurations.
    pub root: Option<String>,

    /// The **meta section**, defining offsets and sizes within the `NDEF`
    /// payload.
    ///
    /// # See also
    /// - https://specs.openprinttag.org/#/nfc_data_format?id=_4-meta-section
    pub meta_fields: Option<String>,

    /// The **main section** contains material information that does not change
    /// during the package instance lifetime.
    ///
    /// # See also
    /// - https://specs.openprinttag.org/#/nfc_data_format?id=_5-main-section
    pub main_fields: Option<String>,

    /// The **auxiliary section** is intended for dynamic data, typically usage
    /// tracking.
    ///
    /// # See also
    /// - https://specs.openprinttag.org/#/nfc_data_format?id=_6-auxiliary-section
    pub aux_fields: Option<String>,
}

impl ConfigStructSchema {
    const NAME: &'static str = "PrintTag";
    const DOC: &'static str = include_str!("docs/print_tag.md");

    const META_DOC: &'static str = "Offsets and sizes within the `NDEF` payload.";

    const MAIN_DOC: &'static str =
        "Contains material information that does not change during the package instance lifetime.";

    const AUX_DOC: &'static str = "Dynamic data, typically usage tracking.";

    pub fn load(context: Context, schema_name: &str) -> crate::Result<Self> {
        let shared_context = SharedContext::new(context);
        let local_context = LocalContext::new(shared_context, schema_name);
        local_context.seed_local().map_err(crate::Error::from)
    }
}

impl GetIdent for ConfigStructSchema {
    fn get_name(&self) -> Option<String> {
        Some(Self::NAME.to_string())
    }
}

impl GetDoc for ConfigStructSchema {
    fn get_doc(&self) -> Option<String> {
        Some(Self::DOC.to_string())
    }
}

impl GetDocAsAttributes for ConfigStructSchema {}

impl GetPubVisibility for ConfigStructSchema {}

impl GetFields for ConfigStructSchema {
    fn get_fields(&self) -> Vec<Field> {
        let mut fields = Vec::new();

        if let Some(meta_schema) = &self.meta_fields {
            let meta_type = name::to_ident(meta_schema);
            fields.push(parse_quote! { pub meta: #meta_type });
        }

        if let Some(main_schema) = &self.main_fields {
            let main_type = name::to_ident(main_schema);
            fields.push(parse_quote! { pub main: #main_type });
        }

        if let Some(aux_schema) = &self.aux_fields {
            let aux_type = name::to_ident(aux_schema);
            fields.push(parse_quote! { pub aux: #aux_type });
        }

        fields
    }
}

impl ToItems for ConfigStructSchema {
    fn to_items(&self) -> Vec<Item> {
        let vis = self.get_visibility();
        let ident = self.get_ident();
        let fields = self.get_fields_punctuated();
        let attrs = self.get_attributes();

        let config_item: Item = parse_quote! {
            #(#attrs)*
            #vis struct #ident {
                #fields
            }
        };

        let mut items = vec![config_item];
        items.extend(builtin::EnumArray.to_items());
        items.extend(builtin::Error.to_items());
        items.extend(self.context.values().flat_map(|s| s.to_items()));

        items
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RawConfigStructSchema {
    pub(crate) mime_type: String,
    pub(crate) root: Option<String>,
    pub(crate) meta_fields: Option<String>,
    pub(crate) main_fields: Option<String>,
    pub(crate) aux_fields: Option<String>,
}

impl<'a> DeserializeWithContext<'a> for ConfigStructSchema {
    fn deserialize_with_context<'de, D>(
        deserializer: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let RawConfigStructSchema { mime_type, root, meta_fields, main_fields, aux_fields } =
            RawConfigStructSchema::deserialize(deserializer)?;

        if let Some(meta_fields) = meta_fields.as_ref() {
            let meta_context = LocalContext::new_from(local_context, meta_fields.as_str())
                .with_description(Self::META_DOC);
            meta_context.load_and_insert_struct().map_err(|e| D::Error::custom(e.to_string()))?;
        }

        if let Some(main_fields) = main_fields.as_ref() {
            let meta_context = LocalContext::new_from(local_context, main_fields.as_str())
                .with_description(Self::MAIN_DOC);
            meta_context.load_and_insert_struct().map_err(|e| D::Error::custom(e.to_string()))?;
        }

        if let Some(aux_fields) = aux_fields.as_ref() {
            let meta_context = LocalContext::new_from(local_context, aux_fields.as_str())
                .with_description(Self::AUX_DOC);
            meta_context.load_and_insert_struct().map_err(|e| D::Error::custom(e.to_string()))?;
        }

        let config_schema = ConfigStructSchema {
            schema_name: local_context.schema_name.to_string(),
            context: local_context.context.clone(),
            mime_type,
            root,
            meta_fields,
            main_fields,
            aux_fields,
        };

        Ok(config_schema)
    }
}
