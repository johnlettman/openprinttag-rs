use serde_norway::Value;
use std::{collections::HashMap, sync::Arc};

/// Remap the top-level keys of a serde mapping value according to `name_map`.
/// Non-mapping values are returned unchanged. The map is modified in-place.
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

        for (old_key, _) in updates.iter() {
            map.remove(old_key);
        }

        for (new_key, new_val) in updates {
            map.insert(new_key, new_val);
        }
    }
}
