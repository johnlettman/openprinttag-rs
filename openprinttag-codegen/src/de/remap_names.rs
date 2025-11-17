use crate::tracing::trace;
use serde_norway::Value;
use std::{collections::HashMap, sync::Arc};

/// Remap the top-level keys of a serde mapping value according to `name_map`.
/// Non-mapping values are returned unchanged. The map is modified in-place.
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", skip(value)))]
pub fn remap_names(value: &mut Value, name_map: Option<Arc<HashMap<String, String>>>) {
    if let (Value::Mapping(ref mut map), Some(rename)) = (value, name_map) {
        let mut updates = Vec::new();

        for (k, v) in map.iter() {
            if let Value::String(ref key_str) = k {
                if let Some(mapped) = rename.get(key_str) {
                    // Avoid renaming to the same key
                    if mapped != key_str {
                        updates.push((Value::String(mapped.clone()), v.clone()));
                    }
                }
            }
        }

        #[cfg(feature = "tracing")]
        trace!(
            "Remapping names:\n{}",
            updates
                .iter()
                .map(|(new_key, new_val)| {
                    use serde_norway::Value;
                    if let (Value::String(ref new_key), Value::String(ref new_val)) =
                        (new_key, new_val)
                    {
                        format!("{} => {}", new_key, new_val)
                    } else {
                        format!("{:?} => {:?}", new_key, new_val)
                    }
                })
                .collect::<Vec<String>>()
                .join("\n")
        );

        trace!("Removing old keys");
        for (old_key, _) in updates.iter() {
            map.remove(old_key);
        }

        trace!("Adding new keys");
        for (new_key, new_val) in updates {
            map.insert(new_key, new_val);
        }
    }
}
