use crate::{
    de::{DeserializeWithContext, WithContext},
    schema::{context::LocalContext, DataEnumVariant},
};
use std::fmt;

pub type DataEnumVariants = Vec<DataEnumVariant>;

impl<'a> DeserializeWithContext<'a> for DataEnumVariants {
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "debug", fields(%schema = local_context.schema_name), skip(deserializer, local_context)))]
    fn deserialize_with_context<'de, D>(
        deserializer: D,
        local_context: &'a LocalContext<'a>,
    ) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{SeqAccess, Visitor};

        struct DataEnumVariantsVisitor<'a>(&'a LocalContext<'a>);

        impl<'de, 'a> Visitor<'de> for DataEnumVariantsVisitor<'a> {
            type Value = Vec<DataEnumVariant>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a sequence of EnumVariantSchema")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut items = Vec::with_capacity(seq.size_hint().unwrap_or(4));

                while let Some(variant) =
                    seq.next_element_seed(WithContext::<DataEnumVariant>::new(self.0))?
                {
                    items.push(variant);
                }

                Ok(items)
            }
        }

        deserializer.deserialize_seq(DataEnumVariantsVisitor(local_context))
    }
}
