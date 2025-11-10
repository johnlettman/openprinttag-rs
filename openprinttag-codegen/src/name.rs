/// Known acronyms and chemical abbreviations that should remain uppercase when
/// converting names to CamelCase identifiers.
///
/// This ensures that terms like `"ptfe"` and `"material_id"` are formatted as
/// `"PTFE"` and `"MaterialID"` instead of `"Ptfe"` or `"MaterialId"`.
pub const ALL_CAPS: &[&str] = &[
    "PTFE", "PVC", "ABS", "PC", "PE", "PP", "PET", "PBT", "PA", "PU", "ESD", "FFF", "SLA", "EMI",
    "ID", "UUID", "GTIN",
];

/// Converts a snake_case or underscore-separated name into a Rust-style
/// CamelCase identifier.
///
/// Known acronyms (from [`ALL_CAPS`]) are preserved in uppercase. For all other
/// segments, the first letter is capitalized and the rest are lowercased.
///
/// # Type Parameters
/// - `S`: Type of the name (typically `str` or [`String`]).
///
/// # Arguments
/// - `name`: Name to convert.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::name;
///
/// assert_eq!(name::to_camel("contains_glass"), "ContainsGlass");
/// assert_eq!(name::to_camel("contains_ptfe"), "ContainsPTFE");
/// assert_eq!(name::to_camel("material_class_enum"), "MaterialClassEnum");
/// ```
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

/// Converts a YAML schema enum filename (e.g., `"material_type_enum.yaml"`)
/// into a Rust-style enum type name (e.g., `"MaterialType"`).
///
/// This strips `.yaml` and `_enum` suffixes before applying [`to_camel`].
///
/// # Type Parameters
/// - `S`: Type of the name (typically `str` or [`String`]).
///
/// # Arguments
/// - `name`: Name to convert.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::name;
///
/// assert_eq!(name::to_enum_name("material_type_enum.yaml"), "MaterialType");
/// ```
pub fn to_enum_name<S: AsRef<str>>(name: S) -> String {
    let name = name.as_ref();
    let name = name.strip_suffix(".yaml").unwrap_or(name);
    let name = name.strip_suffix("_enum").unwrap_or(name);
    to_camel(name)
}

/// Builds a fully qualified Rust enum variant reference (e.g.,
/// `"MaterialType::PC"`).
///
/// The enum name is normalized with [`to_enum_name`], and the enum variant name
/// is normalized with [`to_camel`].
///
/// # Type Parameters
/// - `EN`: Type of the enum name (typically `str` or [`String`]).
/// - `EV`: Type of the enum variant name (typically `str` or [`String`]).
///
/// # Arguments
/// - `name`: Enum name.
/// - `variant`: Enum variant name.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::name;
///
/// assert_eq!(name::to_enum_reference("material_type", "pc"), "MaterialType::PC");
/// ```
#[inline]
pub fn to_enum_reference<EN: AsRef<str>, EV: AsRef<str>>(name: EN, variant: EV) -> String {
    format!("{}::{}", to_enum_name(name), to_camel(variant))
}

/// Builds a Markdown-formatted reference link to a Rust enum variant.
///
/// Produces output like:
/// ```md
/// [`PC`][MaterialType::PC]
/// ```
///
/// # Type Parameters
/// - `EN`: Type of the enum name (typically `str` or [`String`]).
/// - `EV`: Type of the enum variant name (typically `str` or [`String`]).
///
/// # Arguments
/// - `name`: Enum name.
/// - `variant`: Enum variant name.
///
/// This helper is typically used in generated documentation.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::name;
///
/// assert_eq!(name::to_enum_reference_md("material_type", "pc"), "[`PC`][MaterialType::PC]");
/// ```
///
/// # See also
/// - [`to_enum_reference`] for the plain Rust reference.
/// - [`to_enum_references_md`] for linking multiple variants.
#[inline]
pub fn to_enum_reference_md<EN: AsRef<str>, EV: AsRef<str>>(name: EN, variant: EV) -> String {
    format!("[`{}`][{}]", to_camel(variant.as_ref()), to_enum_reference(name, variant))
}

/// Builds a Markdown-formatted, comma-separated list of links to Rust enum
/// variants.
///
/// Each element uses [`to_enum_reference_md`] for consistent formatting.
/// Returns [`None`] if the input list is empty.
///
/// Produces output like:
/// ```md
/// [`PC`][MaterialType::PC], [`ABS`][MaterialType::ABS]
/// ```
///
/// # Type Parameters
/// - `EN`: Type of the enum name (typically `str` or [`String`]).
/// - `EV`: Type of the enum variant name (typically `str` or [`String`]).
///
/// # Arguments
/// - `name`: Enum name.
/// - `variants`: Collection of enum variant names.
///
/// # Example
/// ```rust
/// use openprinttag_codegen::name;
///
/// let links = name::to_enum_references_md("material_type", &vec!["pc", "abs"]).unwrap();
/// assert_eq!(links, "[`PC`][MaterialType::PC], [`ABS`][MaterialType::ABS]");
/// ```
///
/// # See also
/// - [`to_enum_reference_md`] for single-variant formatting.
/// - [`to_enum_reference`] for plain Rust references.
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
