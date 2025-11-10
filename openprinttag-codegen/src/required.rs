use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Copy, Clone, strum::Display, strum::EnumString)]
pub enum Required {
    #[strum(ascii_case_insensitive, to_string = "true")]
    Yes,

    #[strum(ascii_case_insensitive, to_string = "false")]
    No,

    #[strum(ascii_case_insensitive, to_string = "recommended")]
    Recommended
}

impl Default for Required {
    fn default() -> Self {
        Required::No
    }
}

impl From<bool> for Required {
    fn from(value: bool) -> Self {
        if value {
            Required::Yes
        } else {
            Required::No
        }
    }
}

impl<'de> Deserialize<'de> for Required {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VisitorImpl;

        impl<'de> de::Visitor<'de> for VisitorImpl {
            type Value = Required;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a boolean or one of the strings \"true\", \"false\", or \"recommended\"")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(Required::from(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v.trim().to_ascii_lowercase().as_str() {
                    "true" => Ok(Required::Yes),
                    "false" => Ok(Required::No),
                    "recommended" => Ok(Required::Recommended),
                    other => Err(E::invalid_value(de::Unexpected::Str(other), &self)),
                }
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_any(VisitorImpl)
    }
}

impl Serialize for Required {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}