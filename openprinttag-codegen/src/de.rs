use serde::{de, Deserialize, Deserializer};
use serde_norway::{mapping::Entry, Value};
use std::collections::BTreeMap;

/// A mapping of field names used for renaming or field inference.
///
/// The [`NameMap`] is typically used to pass a key–value mapping into functions
/// like [`rename_fields`], [`load_mapped`], or [`load_mapped_from_path`], where
/// each `(new_name, old_name)` pair defines how to rename fields during
/// deserialization.
///
/// # Example
/// ```rust
/// let mut name_map = NameMap::new();
/// name_map.insert("full_name".into(), "display_name".into());
/// name_map.insert("short_name".into(), "name".into());
/// ```
///
/// [`load_mapped`]: crate::data::load_mapped
/// [`load_mapped_from_path`]: crate::data::load_mapped_from_path
pub type NameMap = BTreeMap<String, String>;

/// Recursively rename keys in a [`Value`] according to a provided mapping of
/// new-from-old (e.g., `<new, old>`) key names.
///
/// This function traverses the input [`Value`] tree, including all nested
/// mappings and sequences, and renames any object keys that match entries in
/// `name_map`. The mapping is applied at every level, allowing bulk field
/// renaming in complex YAML documents (such as [OpenPrintTag] schema files).
///
/// The function operates **in-place**, modifying the provided `value` directly.
///
/// # Type Parameters
/// - `K`: Type of the *new* key name (typically `str` or [`String`])
/// - `V`: Type of the *old* key name (typically `str` or [`String`])
/// - `C`: The name map container, which must implement `IntoIterator<Item =
///   (&K, &V)>`
///
/// # Arguments
/// - `value`: A mutable reference to a [`Value`] structure (mapping, sequence,
///   or scalar).
/// - `name_map`: A mapping of new-from-old key names to apply recursively.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::de::rename_fields;
/// use serde_norway::Value;
/// use std::collections::BTreeMap;
///
/// #[derive(Debug, Clone, serde::Deserialize)]
/// struct MyData {
///     pub real_name: String,
///     pub real_display_name: String,
/// }
///
/// // load the YAML as a Value
/// let mut value = serde_norway::from_str::<Value>(
///     r#"
/// name: GF
/// display_name: "Glass Fiber"
/// "#,
/// )
/// .expect("should load YAML string");
///
/// // create a mapping of new-from-old names
/// let mut name_map = BTreeMap::new();
/// name_map.insert("real_name", "name");
/// name_map.insert("real_display_name", "display_name");
///
/// // rename the fields and load the value into your struct
/// rename_fields(&mut value, &name_map);
/// let data: MyData = serde_norway::from_value(value).expect("should load YAML value");
///
/// assert_eq!(data.real_name, "GF");
/// assert_eq!(data.real_display_name, "Glass Fiber");
/// ```
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

/// Custom deserializer for optional multi-line description text matching the
/// conventions of [OpenPrintTag].
///
/// This function is designed to parse description fields that may be
/// represented in several forms:
/// - As a plain string (`"Contains glass"`).
/// - As a sequence of strings (`["Contains glass fibers.", "Strong and
///   rigid."]`).
/// - As `null` or missing.
///
/// It deserializer normalizes these inputs into a single [`Option`] [`String`]
/// value:
/// - `null` or a missing field: [`None`].
/// - A single string: [`Some`] [`String`].
/// - A list of strings: [`Some`] [`String`] with the list joined using
///   paragraph and line break conventions matching Rust documentation strings.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::de::description;
///
/// #[derive(Debug, Clone, serde::Deserialize)]
/// struct MyData {
///     #[serde(default, deserialize_with = "description")]
///     description: Option<String>,
/// }
///
/// // single-line
/// let single: MyData = serde_norway::from_str(r#"description: "Contains glass""#)
///     .expect("should load YAML string");
/// assert_eq!(single.description, Some("Contains glass".into()));
///
/// // multi-line
/// let multi: MyData = serde_norway::from_str(
///     r#"
/// description:
///   - "Contains glass fibers."
///   - "Strong and rigid."
/// "#,
/// )
/// .expect("should load YAML string");
/// assert_eq!(multi.description, Some("Contains glass fibers.\n\nStrong and rigid.".into()));
///
/// // null field
/// let missing: MyData =
///     serde_norway::from_str(r#"description: null"#).expect("should load YAML string");
/// assert_eq!(missing.description, None);
///
/// // empty
/// let empty: MyData =
///     serde_norway::from_str(r#"description: "       ""#).expect("should load YAML string");
/// assert_eq!(empty.description, None);
/// ```
///
/// # See also
/// - [`ser::description`][crate::ser::description] for the inverse
///   serialization logic.
///
/// [OpenPrintTag]: https://openprinttag.org/
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
            let value = value.trim();
            Ok((!value.is_empty()).then_some(value.to_string()))
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
