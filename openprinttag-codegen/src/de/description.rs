use serde::{de, Deserializer};
use std::fmt;

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

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
