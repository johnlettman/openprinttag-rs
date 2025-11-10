use serde::{de, Deserialize, Deserializer};
use serde_norway::mapping::Entry;
use serde_norway::Value;

pub fn rename_fields<K, V, C>(value: &mut Value, name_map: &C)
where
    K: AsRef<str>,
    V: AsRef<str>,
    for<'a> &'a C: IntoIterator<Item = (&'a K, &'a V)>,
{
    match value {
        Value::Mapping(map) => {
            // collect all updates before modifying the map
            let mut updates: Vec<(Value, Value)> = Vec::new();

            for (new_key, old_key) in name_map {
                let old_key = old_key.as_ref();
                let new_key = new_key.as_ref();

                // look for an entry under the old key
                if let Entry::Occupied(entry) = map.entry(Value::String(old_key.to_string())) {
                    let old_value = entry.get().clone();
                    updates.push((Value::String(new_key.to_string()), old_value));
                    map.remove(Value::String(old_key.to_string()));
                }
            }

            // insert updated entries
            for (new_key, value) in updates {
                map.insert(new_key, value);
            }

            // recurse into children
            for (_k, v) in map.iter_mut() {
                rename_fields(v, name_map);
            }
        },

        Value::Sequence(seq) => {
            for v in seq.iter_mut() {
                rename_fields(v, name_map);
            }
        },

        _ => {},
    }
}

pub fn none_as_option<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(s) if s.eq_ignore_ascii_case("none") => Ok(None),
        Some(s) => Ok(Some(s)),
    }
}

pub fn description<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DescriptionVisitor;

    impl<'de> de::Visitor<'de> for DescriptionVisitor {
        type Value = Option<String>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a string, a list of strings, or null")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(value.trim().to_string()))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut lines = Vec::new();
            while let Some(entry) = seq.next_element::<String>()? {
                lines.push(entry.trim().to_string());
            }

            if lines.is_empty() {
                return Ok(None);
            }

            let mut result = String::new();
            for (i, line) in lines.iter().enumerate() {
                if i == 0 {
                    result.push_str(line);
                } else if i == 1 {
                    result.push_str("\n\n");
                    result.push_str(line);
                } else {
                    result.push('\n');
                    result.push_str(line);
                }
            }

            Ok(Some(result))
        }
    }

    deserializer.deserialize_any(DescriptionVisitor)
}
