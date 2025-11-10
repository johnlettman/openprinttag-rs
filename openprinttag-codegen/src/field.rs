use crate::{data, gen, name, FieldType, Required};
use serde::{Deserialize, Deserializer};

pub type Fields = Vec<Field>;

#[derive(Debug, Clone)]
pub struct Field {
    key: u32,
    name: Option<String>,
    description: Option<String>,

    required: Option<Required>,
    deprecated: bool,

    field_type: Option<FieldType>,
}

impl Field {
    pub fn make_field(&self) -> Option<syn::Field> {
        use syn::FieldMutability;

        let attrs = self
            .description
            .as_ref()
            .and_then(|d| gen::make_doc_attribute(d).ok())
            .into_iter()
            .collect::<Vec<_>>();

        let vis = gen::make_pub_visibility();

        let name = self.name.as_ref()?;
        let ident = Some(gen::make_ident(name));

        let field_type = self.field_type.as_ref()?;
        let ty = field_type.make_type().ok()?;

        Some(syn::Field {
            attrs,
            vis,
            mutability: FieldMutability::None,
            ident,
            colon_token: Default::default(),
            ty,
        })
    }
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error as _;

        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub(crate) struct RawField {
            pub(crate) key: u32,

            #[serde(default)]
            pub(crate) name: Option<String>,

            #[serde(default)]
            pub(crate) description: Option<serde_norway::Value>,

            #[serde(default)]
            pub(crate) required: Option<Required>,

            #[serde(default)]
            pub(crate) deprecated: bool,

            #[serde(default)]
            pub(crate) r#type: Option<String>,

            #[serde(default)]
            pub(crate) max_length: Option<u64>,

            #[serde(default)]
            pub(crate) unit: Option<String>,

            #[serde(default)]
            pub(crate) items_file: Option<String>,

            #[serde(default)]
            pub(crate) name_field: Option<String>,

            // https://github.com/prusa3d/OpenPrintTag/issues/78
            #[serde(default)]
            pub(crate) full_name_field: Option<String>,

            #[serde(default)]
            pub(crate) example: Option<serde_norway::Value>,
        }

        let raw = RawField::deserialize(deserializer)?;

        let description = if let Some(val) = raw.description {
            crate::de::description(val).map_err(D::Error::custom)?
        } else {
            None
        };

        let field_type = if let Some(r#type) = raw.r#type {
            Some(match r#type.as_str() {
                "uuid" => FieldType::UUID,
                "string" => FieldType::String(raw.max_length.unwrap_or(255) as usize),
                "bytes" => FieldType::Bytes(raw.max_length.unwrap_or(255) as usize),
                "int" | "integer" => {
                    let example = if let Some(example) = raw.example {
                        example.as_u64().map(|i| i as u32)
                    } else {
                        None
                    };

                    FieldType::Integer { unit: raw.unit, example }
                },
                "number" => {
                    let example = if let Some(example) = raw.example {
                        example.as_f64().map(|f| f as f32)
                    } else {
                        None
                    };

                    FieldType::Number { unit: raw.unit, example }
                },
                "timestamp" => FieldType::Timestamp,
                "enum" => {
                    let items_file =
                        raw.items_file.ok_or_else(|| D::Error::missing_field("items_file"))?;

                    let name = name::to_enum_name(&items_file);
                    let variants = data::load_enum_variants(items_file, raw.name_field)
                        .map_err(|e| D::Error::custom(e))?;

                    FieldType::Enum { name, description: description.clone(), variants }
                },
                "enum_array" => {
                    let items_file =
                        raw.items_file.ok_or_else(|| D::Error::missing_field("items_file"))?;

                    let name = name::to_enum_name(&items_file);
                    let variants = data::load_enum_variants(items_file, raw.name_field)
                        .map_err(|e| D::Error::custom(e))?;

                    FieldType::EnumArray { name, description: description.clone(), variants }
                },
                other => return Err(D::Error::custom(format!("Unknown type: {}", other))),
            })
        } else {
            None
        };

        Ok(Field {
            key: raw.key,
            name: raw.name,
            description,
            required: raw.required,
            deprecated: raw.deprecated,
            field_type,
        })
    }
}