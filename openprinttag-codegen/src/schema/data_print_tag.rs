use serde::{de::Error, Deserialize, Deserializer, Serialize};
use syn::{parse_quote, Field, Item};

use crate::{
    de::DeserializeWithContext,
    schema::{
        builtin,
        context::{registry::Register, Context, LocalContext, SharedContext},
        name, GetSchemas, Schema,
    },
};
use crate::emit::{EmitFields, EmitItems, EmitPubVisibility, EmitVisibility, EmitIdent, EmitDocsAsAttributes, EmitAttributes};
use crate::emit::util::emit_doc_attribute;
use crate::schema::HasDocs;

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
pub struct DataPrintTag {
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

impl DataPrintTag {
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

impl GetSchemas for DataPrintTag {
    fn get_schemas(&self) -> Vec<Schema> {
        self.context.values().collect()
    }
}

impl EmitIdent for DataPrintTag {
    fn get_name(&self) -> Option<String> {
        Some(Self::NAME.to_string())
    }
}

impl HasDocs for DataPrintTag {
    fn docs(&self) -> Option<String> {
        Some(Self::DOC.to_string())
    }
}

impl EmitPubVisibility for DataPrintTag {}

impl EmitFields for DataPrintTag {
    fn emit_core_fields(&self) -> Vec<Field> {
        let mut fields = Vec::new();

        macro_rules! push_fields {
            ($fields:expr, $schema:expr, $field:ident, $doc:expr) => {
                if let Some(schema) = $schema.as_ref() {
                    let ty = name::to_camel_ident(schema);
                    let doc_attr = emit_doc_attribute($doc);
                    $fields.push(parse_quote! {
                        #doc_attr
                        pub $field: #ty
                    });
                }
            };
        }

        push_fields!(fields, self.meta_fields, meta, Self::META_DOC);
        push_fields!(fields, self.main_fields, main, Self::MAIN_DOC);
        push_fields!(fields, self.aux_fields, aux, Self::AUX_DOC);

        fields
    }
}

impl EmitDocsAsAttributes for DataPrintTag {}

impl EmitItems for DataPrintTag {
    fn emit_core_items(&self) -> Vec<Item> {
        let vis = self.emit_visibility();
        let ident = self.rs_core_ident();
        let fields = self.emit_core_fields();
        let attrs = self.emit_core_attributes();

        let mut items = vec![
            parse_quote! {
                #(#attrs)*
                #vis struct #ident {
                    #(#fields),*
                }
            },
        ];
        items.extend(builtin::EnumArray.emit_core_items());
        items.extend(builtin::Error.emit_core_items());
        items.extend(builtin::Timestamp.emit_core_items());
        items.extend(builtin::Uuid.emit_core_items());
        items.extend(self.context.values().flat_map(|s| s.emit_core_items()));

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

impl<'a> DeserializeWithContext<'a> for DataPrintTag {
    fn deserialize_with_context<'de, D>(
        deserializer: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let RawConfigStructSchema { mime_type, root, meta_fields, main_fields, aux_fields } =
            RawConfigStructSchema::deserialize(deserializer)?;

        macro_rules! load_struct {
            ($schema:expr, $doc:expr) => {
                if let Some(name) = $schema.as_ref() {
                    let context =
                        LocalContext::new_from(local_context, name.as_str()).with_description($doc);
                    context.load_and_insert_struct().map_err(|e| {
                        D::Error::custom(format!("failed to load schema '{}': {}", name, e))
                    })?;
                }
            };
            () => {};
        }

        load_struct!(meta_fields, Self::META_DOC);
        load_struct!(main_fields, Self::MAIN_DOC);
        load_struct!(aux_fields, Self::AUX_DOC);

        let config_schema = DataPrintTag {
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
