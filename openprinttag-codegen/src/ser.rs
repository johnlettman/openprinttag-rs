use serde::{Serialize, Serializer};

/// Custom serializer for optional multi-line description text matching the conventions of
/// [OpenPrintTag].
///
/// This function is designed to convert an [`Option`] [`String`] description field into either a
/// single string or a YAML list of strings, depending on its contents:
/// - If the value is [`None`], it serializes as `null`.
/// - If the value is a single paragraph (no double newlines or line breaks), it serializes as a
///     plain string.
/// * If the value contains multiple paragraphs or line breaks, it splits them and serializes as a
///     sequence of strings.
///
/// Consecutive blank lines are treated as paragraph separators (`"\n\n"`), while single newlines
/// are treated as intra-paragraph line breaks. Leading and trailing whitespace are trimmed from
/// each line.
///
/// [OpenPrintTag]: https://openprinttag.org/
pub fn description<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        None => serializer.serialize_none(),
        Some(text) => {
            let parts: Vec<&str> = text
                .split("\n\n")
                .flat_map(|paragraph| paragraph.split('\n'))
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            if parts.len() == 1 {
                serializer.serialize_str(parts[0])
            } else {
                parts.serialize(serializer)
            }
        }
    }
}
