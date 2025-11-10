use serde::{Serialize, Serializer};

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
