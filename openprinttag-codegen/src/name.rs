pub const ALL_CAPS: &[&str] = &[
    "PTFE", "PVC", "ABS", "PC", "PE", "PP", "PET", "PBT", "PA", "PU", "ESD", "FFF", "SLA", "EMI",
    "ID", "UUID", "GTIN",
];

pub fn to_camel<S: AsRef<str>>(name: S) -> String {
    name.as_ref()
        .split('_')
        .map(|part| {
            if let Some(&upper) = ALL_CAPS.iter().find(|&&u| u.eq_ignore_ascii_case(part)) {
                upper.to_string()
            } else {
                let mut chars = part.chars();
                match chars.next() {
                    Some(first) => first.to_ascii_uppercase().to_string() + &chars.as_str(),
                    None => String::new(),
                }
            }
        })
        .collect()
}

pub fn to_enum_name<S: AsRef<str>>(name: S) -> String {
    let name = name.as_ref();
    let name = name.strip_suffix(".yaml").unwrap_or(name);
    let name = name.strip_suffix("_enum").unwrap_or(name);
    to_camel(name)
}

pub fn to_enum_reference<EN: AsRef<str>, EV: AsRef<str>>(name: EN, variant: EV) -> String {
    format!("{}::{}", to_camel(name), to_camel(variant))
}

pub fn to_enum_references<EN: AsRef<str>, EV: AsRef<str>>(
    name: EN,
    variants: &Vec<EV>,
) -> Option<String> {
    (!variants.is_empty()).then_some(
        variants.iter().map(|variant| to_enum_reference(name.as_ref(), variant)).collect(),
    )
}

pub fn to_enum_reference_md<EN: AsRef<str>, EV: AsRef<str>>(name: EN, variant: EV) -> String {
    let name = to_camel(name);
    format!("[`{}`][{}::{}]", name, name, to_camel(variant))
}

pub fn to_enum_references_md<EN: AsRef<str>, EV: AsRef<str>>(
    name: EN,
    variants: &Vec<EV>,
) -> Option<String> {
    (!variants.is_empty()).then_some(
        variants
            .iter()
            .map(|variant| to_enum_reference_md(name.as_ref(), variant))
            .collect::<Vec<_>>()
            .join(", "),
    )
}
