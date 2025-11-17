use crate::{
    de::{DeserializeWithContext, WithContext},
    schema::{context::LocalContext, DataStructField},
};
use std::fmt;

pub type DataStructFields = Vec<DataStructField>;

impl<'a> DeserializeWithContext<'a> for DataStructFields {
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", fields(%schema = local_context.schema_name), skip(deserializer, local_context)))]
    fn deserialize_with_context<'de, D>(
        deserializer: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{SeqAccess, Visitor};

        struct StructureFieldsVisitor<'a>(&'a LocalContext<'a>);

        impl<'de, 'a> Visitor<'de> for StructureFieldsVisitor<'a> {
            type Value = Vec<DataStructField>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a sequence of StructFieldSchema")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(4));

                while let Some(variant) =
                    seq.next_element_seed(WithContext::<DataStructField>::new(self.0))?
                {
                    items.push(variant);
                }

                Ok(items)
            }
        }

        deserializer.deserialize_seq(StructureFieldsVisitor(local_context))
    }
}
