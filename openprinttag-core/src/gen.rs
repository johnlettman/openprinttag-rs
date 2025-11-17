/**Defines an [OpenPrintTag] with the [OpenPrintTag data layout], composed
of three major field groups:

- **Meta section** (`meta_fields`): Defines region offsets and sizes within
  the NDEF payload.
- **Main section** (`main_fields`): Contains static material information.
- **Auxiliary section** (`aux_fields`): Contains dynamic runtime or
  usage-tracking data.

# Structure
| Section       | Purpose                                                   | Reference           |
|---------------|-----------------------------------------------------------|---------------------|
| **Meta**      | Defines offsets, sizes, and low-level NFC layout details. | [Meta section]      |
| **Main**      | Defines static, per-material properties.                  | [Main section]      |
| **Auxiliary** | Defines dynamic or runtime tracking data.                 | [Auxiliary section] |

[Meta section]: https://specs.openprinttag.org/#/nfc_data_format?id=_4-meta-section
[Main section]: https://specs.openprinttag.org/#/nfc_data_format?id=_5-main-section
[Auxiliary section]: https://specs.openprinttag.org/#/nfc_data_format?id=_6-auxiliary-section
[OpenPrintTag]: https://openprinttag.org/
[OpenPrintTag data layout]: https://specs.openprinttag.org/#/nfc_data_format
*/
pub struct PrintTag {
    ///Offsets and sizes within the `NDEF` payload.
    pub meta: Meta,
    ///Contains material information that does not change during the package instance lifetime.
    pub main: Main,
    ///Dynamic data, typically usage tracking.
    pub aux: Aux,
}
#[cfg(feature = "std")]
/**An array of enum values.
*/
pub type EnumArray<T, const N: usize> = std::vec::Vec<T>;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
/**An array of enum values.
*/
///(no `std`, using `alloc`)
pub type EnumArray<T, const N: usize> = alloc::vec::Vec<T>;
#[cfg(all(not(feature = "std"), not(feature = "alloc")))]
/**An array of enum values.
*/
///(no `std` and no `alloc`)
pub type EnumArray<T, const N: usize> = heapless::Vec<T, N>;
#[derive(Debug, thiserror::Error)]
///OpenPrintTag errors.
pub enum Error {
    #[error("unexpected type")]
    UnexpectedType,
    #[error("invalid enum discriminant: {0}")]
    InvalidEnumDiscriminant(u16),
    #[error("too many items in the array")]
    TooManyItems,
    #[error("missing field: {0}")]
    MissingField(&'static str),
    #[error("CBOR decode error")]
    DecodeError(minicbor::decode::Error),
}
impl From<minicbor::decode::Error> for Error {
    #[inline(always)]
    fn from(e: minicbor::decode::Error) -> Self {
        Self::DecodeError(e)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Timestamp(pub u32);
impl Timestamp {
    #[inline(always)]
    pub const fn new(secs: u32) -> Self {
        Self(secs)
    }
    #[inline(always)]
    pub const fn as_secs(self) -> u32 {
        self.0
    }
    #[cfg(feature = "std")]
    pub fn now() -> Self {
        use core::cmp::min;
        use std::time::{SystemTime, UNIX_EPOCH};
        let dur =
            SystemTime::now().duration_since(UNIX_EPOCH).expect("system time is before Unix epoch");
        let secs = min(dur.as_secs(), u32::MAX as u64) as u32;
        Self(secs)
    }
}
impl core::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(all(feature = "chrono", feature = "std"))]
        {
            let dt: chrono::DateTime<chrono::Utc> = (*self).into();
            return write!(f, "{}", dt.to_rfc3339());
        }
        #[cfg(all(feature = "time", not(feature = "chrono")))]
        {
            let dt: time::OffsetDateTime = (*self).into();
            return write!(
                f,
                "{}",
                dt.format(&time::format_description::well_known::Rfc3339).unwrap()
            );
        }
        #[cfg(all(feature = "std", not(feature = "chrono"), not(feature = "time")))]
        {
            use std::time::{Duration, UNIX_EPOCH};
            let systime = UNIX_EPOCH + Duration::from_secs(self.0 as u64);
            return write!(f, "{} (system)", self.0);
        }
        #[cfg(all(not(feature = "std"), not(feature = "chrono"), not(feature = "time")))]
        {
            return write!(f, "{}", self.0);
        }
    }
}
impl Default for Timestamp {
    #[cfg(feature = "std")]
    #[inline(always)]
    fn default() -> Self {
        Self::now()
    }
    #[cfg(not(feature = "std"))]
    #[inline(always)]
    fn default() -> Self {
        Self(0)
    }
}
impl From<Timestamp> for u32 {
    #[inline(always)]
    fn from(ts: Timestamp) -> u32 {
        ts.0
    }
}
impl From<u32> for Timestamp {
    #[inline(always)]
    fn from(secs: u32) -> Self {
        Self(secs)
    }
}
impl<C> minicbor::Encode<C> for Timestamp {
    fn encode<W>(
        &self,
        e: &mut minicbor::Encoder<W>,
        c: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>>
    where
        W: minicbor::encode::Write,
    {
        minicbor::Encode::<C>::encode(&self.0, e, c)
    }
    fn is_nil(&self) -> bool {
        minicbor::Encode::<C>::is_nil(&self.0)
    }
}
impl<'b, C> minicbor::Decode<'b, C> for Timestamp {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        c: &mut C,
    ) -> core::result::Result<Self, minicbor::decode::Error> {
        Ok(Self(minicbor::Decode::<C>::decode(d, c)?))
    }
    fn nil() -> Option<Self> {
        minicbor::Decode::<C>::nil().map(Self)
    }
}
#[cfg(feature = "std")]
impl From<Timestamp> for std::time::SystemTime {
    #[inline]
    fn from(ts: Timestamp) -> std::time::SystemTime {
        use std::time::{Duration, UNIX_EPOCH};
        UNIX_EPOCH + Duration::from_secs(ts.0 as u64)
    }
}
#[cfg(all(feature = "chrono", feature = "std"))]
impl From<Timestamp> for chrono::DateTime<chrono::Utc> {
    fn from(ts: Timestamp) -> Self {
        chrono::DateTime::from_timestamp(ts.0 as i64, 0).expect("invalid Unix timestamp")
    }
}
#[cfg(all(feature = "chrono", feature = "std"))]
impl TryFrom<chrono::DateTime<chrono::Utc>> for Timestamp {
    type Error = std::num::TryFromIntError;
    fn try_from(dt: chrono::DateTime<chrono::Utc>) -> Result<Self, Self::Error> {
        let secs = dt.timestamp();
        u32::try_from(secs).map(Self)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Uuid(pub uuid::Uuid);
impl Uuid {
    pub const BRAND_NAMESPACE: uuid::Uuid = uuid::uuid!("5269dfb7-1559-440a-85be-aba5f3eff2d2");
    pub const MATERIAL_NAMESPACE: uuid::Uuid = uuid::uuid!("616fc86d-7d99-4953-96c7-46d2836b9be9");
    pub const PACKAGE_NAMESPACE: uuid::Uuid = uuid::uuid!("6f7d485e-db8d-4979-904e-a231cd6602b2");
    pub const INSTANCE_NAMESPACE: uuid::Uuid = uuid::uuid!("31062f81-b5bd-4f86-a5f8-46367e841508");
    #[inline(always)]
    pub const fn new(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
    #[inline(always)]
    pub const fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
    #[inline(always)]
    pub const fn into_uuid(self) -> uuid::Uuid {
        self.0
    }
    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }
    #[inline(always)]
    pub fn is_nil(&self) -> bool {
        self.0.is_nil()
    }
    #[inline(always)]
    pub fn derive(namespace: &uuid::Uuid, bytes: &[u8]) -> Self {
        Self(uuid::Uuid::new_v5(namespace, bytes))
    }
    #[inline]
    pub fn derive_uuid(namespace: &uuid::Uuid, uuid: uuid::Uuid) -> Self {
        Self::derive(namespace, uuid.as_bytes())
    }
    pub fn derive_brand_uuid<B: AsRef<str>>(brand_name: B) -> Self {
        let mut buf = [0u8; 271usize];
        let mut offset = 0;
        let bytes = brand_name.as_ref().as_bytes();
        buf[offset..offset + bytes.len()].copy_from_slice(bytes);
        offset += bytes.len();
        Self::derive(&Self::BRAND_NAMESPACE, &buf[..offset])
    }
    pub fn derive_material_uuid<M: AsRef<str>>(brand_uuid: &uuid::Uuid, material_name: M) -> Self {
        let mut buf = [0u8; 287usize];
        let mut offset = 0;
        let bytes = brand_uuid.as_bytes();
        buf[offset..offset + 16].copy_from_slice(bytes);
        offset += 16;
        let bytes = material_name.as_ref().as_bytes();
        buf[offset..offset + bytes.len()].copy_from_slice(bytes);
        offset += bytes.len();
        Self::derive(&Self::MATERIAL_NAMESPACE, &buf[..offset])
    }
    pub fn derive_package_uuid<G: AsRef<str>>(brand_uuid: &uuid::Uuid, gtin: G) -> Self {
        let mut buf = [0u8; 287usize];
        let mut offset = 0;
        let bytes = brand_uuid.as_bytes();
        buf[offset..offset + 16].copy_from_slice(bytes);
        offset += 16;
        let bytes = gtin.as_ref().as_bytes();
        buf[offset..offset + bytes.len()].copy_from_slice(bytes);
        offset += bytes.len();
        Self::derive(&Self::PACKAGE_NAMESPACE, &buf[..offset])
    }
    pub fn derive_instance_uuid(nfc_tag_uid: &[u8; 7]) -> Self {
        let mut buf = [0u8; 23usize];
        let mut offset = 0;
        buf[offset..offset + 7].copy_from_slice(nfc_tag_uid);
        offset += 7;
        Self::derive(&Self::INSTANCE_NAMESPACE, &buf[..offset])
    }
}
impl core::fmt::Display for Uuid {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
impl Default for Uuid {
    #[inline(always)]
    fn default() -> Self {
        Self(uuid::Uuid::nil())
    }
}
impl From<[u8; 16]> for Uuid {
    #[inline]
    fn from(bytes: [u8; 16]) -> Self {
        Self(uuid::Uuid::from_bytes(bytes))
    }
}
impl From<Uuid> for [u8; 16] {
    #[inline(always)]
    fn from(u: Uuid) -> Self {
        *u.as_bytes()
    }
}
impl core::str::FromStr for Uuid {
    type Err = uuid::Error;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        uuid::Uuid::parse_str(s).map(Self)
    }
}
impl From<Uuid> for uuid::Uuid {
    #[inline(always)]
    fn from(v: Uuid) -> uuid::Uuid {
        v.0
    }
}
impl From<uuid::Uuid> for Uuid {
    #[inline(always)]
    fn from(v: uuid::Uuid) -> Self {
        Self(v)
    }
}
impl<C> minicbor::Encode<C> for Uuid {
    fn encode<W>(
        &self,
        e: &mut minicbor::Encoder<W>,
        c: &mut C,
    ) -> core::result::Result<(), minicbor::encode::Error<W::Error>>
    where
        W: minicbor::encode::Write,
    {
        let bytes = self.as_bytes();
        minicbor::Encode::<C>::encode(bytes, e, c)
    }
}
impl<'b, C> minicbor::Decode<'b, C> for Uuid {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        c: &mut C,
    ) -> core::result::Result<Self, minicbor::decode::Error> {
        let bytes: [u8; 16] = minicbor::Decode::decode(d, c)?;
        Ok(Self(uuid::Uuid::from_bytes(bytes)))
    }
    fn nil() -> core::option::Option<Self> {
        <[u8; 16] as minicbor::Decode<C>>::nil().map(|bytes| Self(uuid::Uuid::from_bytes(bytes)))
    }
}
#[derive(Debug, Clone)]
///Contains material information that does not change during the package instance lifetime.
pub struct Main {
    /**Unique identifier of the package instance.

    If not specified, can be deduced from `brand_uuid` + NFC tag UID.
    See _UUID_ section for more details.*/
    pub instance_uuid: Uuid,
    /**Universally unique identifier of the package (product)

    If not specified, can be deduced from `brand_uuid` + `gtin`.
    See _UUID_ section for more details.*/
    pub package_uuid: Uuid,
    /**Universally unique identifier of the material.

    If not specified, can be deduced from `brand_uuid` + `material_name`.
    See _UUID_ section for more details.*/
    pub material_uuid: Uuid,
    /**Universally unique identifier of the brand

    If not specified, can be deduced from the `brand_name` string.
    See _UUID_ section for more details.*/
    pub brand_uuid: Uuid,
    ///Global Trade Item Number.
    pub gtin: f32,
    /**Brand-specific identifier of the package instance.

    Not much use cases at this moment, possibly just for URL deduction*/
    pub brand_specific_instance_id: String,
    /**Brand-specific identifier of the package (product ID).

    Not much use cases at this moment, possibly just for URL deduction*/
    pub brand_specific_package_id: String,
    /**Together with brand uniquely identifies each material.

    Not much use cases at this moment, possibly just for URL deduction.*/
    pub brand_specific_material_id: String,
    pub material_class: MaterialClass,
    /**Coarse classification of the material.

    Useful for determining default parameters for preheat an such that are not explicitly specified in the data.
    If the material does not match any of the proposed material types, can be left unspecified.*/
    pub material_type: MaterialType,
    /**Brand-specific material display string/identifier.

    In the UI, brand_name + material_name should be displayed together, for example "Prusament PLA Galaxy Black".*/
    pub material_name: String,
    /**Abbreviation of the material name, for UI purposes (footers, dashboards, ...).

    If not present, the material inherits the abbreviation from the material type.*/
    pub material_abbreviation: String,
    ///Brand of the material.
    pub brand_name: String,
    /**Indicates whether the tag is write protected (everything except aux section, that one should be always writable).

    See the _Write protection_ section.*/
    pub write_protection: WriteProtection,
    pub manufactured_date: Timestamp,
    ///Country the [MaterialPackageInstance](terminology) was produced in, encoded as a two-letter code according to [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    pub country_of_origin: String,
    pub expiration_date: Timestamp,
    /**Nominal/advertised weight of the full package of the material, excluding the container.

    The actual netto weight of a specific package instance can slightly differ and is specified by `actual_netto_full_weight`.*/
    pub nominal_netto_full_weight: f32,
    /**Actual weight of the full package of the material of this specific package instance, excluding the weight of the container.

    Can slightly differ from `nominal_netto_full_weight`.
    If not present, it is assumed to match `nominal_netto_full_weight`.*/
    pub actual_netto_full_weight: f32,
    /**Nominal/advertised filament length of the full spool.

    The actual length of a specific package instance can slightly differ and is specified by `actual_full_length`*/
    pub nominal_full_length: f32,
    /**Actual filament length of the full spool.

    Can slightly differ from `netto_full_length`.*/
    pub actual_full_length: f32,
    ///Weight of the empty container.
    pub empty_container_weight: f32,
    /**Primary color of the material in the RGB(A) format, intended for UI purposes.

    The alpha channel can be left out, in which case the data should have 3 bytes instead of 4 and the color will be considered fully opaque.
    If a material doesn't have a single primary color (for example rainbow or coextruded filaments), this field can be null.*/
    pub primary_color: [u8; 4],
    /**One of secondary colors of the material.

    Data format is the same as for `primary_color`.*/
    pub secondary_color_0: [u8; 4],
    ///See `secondary_color_0`.
    pub secondary_color_1: [u8; 4],
    ///See `secondary_color_0`.
    pub secondary_color_2: [u8; 4],
    ///See `secondary_color_0`.
    pub secondary_color_3: [u8; 4],
    ///See `secondary_color_0`.
    pub secondary_color_4: [u8; 4],
    /**Transmission Distance is a number representing material opacity.

    Value ranges from 0.1 (least transparent/most opaque) to 100 (most transparent/least opaque).
    See [Prusa TD values](https://help.prusa3d.com/article/hueforge-filament-transparency-values-and-hexcodes_762314) or [HueForge website](https://shop.thehueforge.com/blogs/news/what-is-hueforge).*/
    pub transmission_distance: f32,
    ///Properties of the material. Can have multiple tags at once.
    pub tags: EnumArray<Tags, 71>,
    ///Density of the material.
    pub density: f32,
    /**Diameter of the filament, in mm.

    If not present, 1.75 mm is assumed.*/
    pub filament_diameter: f32,
    /**Hardness of the material on the Shore A hardness scale (suitable for softer materials).

    **Note:** There is no 1:1 mapping between A and D scales, different materials can have different values on one scale even though they are the same on the other.*/
    pub shore_hardness_a: u32,
    /**Hardness of the material on the Shore D hardness scale (suitable for harder materials).

    **Note:** There is no 1:1 mapping between A and D scales, different materials can have different values on one scale even though they are the same on the other.*/
    pub shore_hardness_d: u32,
    /**Filaments can contain particles that would clog smaller nozzles.

    This field specifies minimum nozzle diameter recommended for printing this material.*/
    pub min_nozzle_diameter: f32,
    /**Minimum recommended nozzle temperature for printing.

    Also used for loading the filament to the nozzle.*/
    pub min_print_temperature: u32,
    /**Maximum recommended nozzle temperature for printing.

    Also used for loading the filament to the nozzle.*/
    pub max_print_temperature: u32,
    /**Recommended nozzle temperature for preheating/load cell bed leveling.

    Should be large enough for the material to get soft, but not low enough for it no to drip out of the nozzle.*/
    pub preheat_temperature: u32,
    ///Minimum recommended heatbed temperature.
    pub min_bed_temperature: u32,
    ///Maximum recommended heatbed temperature.
    pub max_bed_temperature: u32,
    ///Minimum recommended temperature of the chamber.
    pub min_chamber_temperature: u32,
    ///Maximum recommended temperature of the chamber.
    pub max_chamber_temperature: u32,
    ///Ideal chamber temperature for printing.
    pub chamber_temperature: u32,
    ///Width of the filament spool. Can be useful to know for spool holders, dry boxes and such.
    pub container_width: u32,
    ///Diameter of the spool. Can be useful to know for spool holders, dry boxes and such.
    pub container_outer_diameter: u32,
    /**Diameter of the inner cylinder the filament is spooled once.

    Equals to the minimum diameter of the filament winding.*/
    pub container_inner_diameter: u32,
    ///Diameter of the center hole of the spool.
    pub container_hole_diameter: u32,
    ///Viscosity of the material at 18 °C.
    pub viscosity_18c: f32,
    ///Viscosity of the material at 25 °C.
    pub viscosity_25c: f32,
    ///Viscosity of the material at 40 °C.
    pub viscosity_40c: f32,
    ///Viscosity of the material at 60 °C.
    pub viscosity_60c: f32,
    ///Maximum amount of material the container can hold.
    pub container_volumetric_capacity: f32,
    ///Wavelength of the light the material has been designed to be cured with.
    pub cure_wavelength: u32,
}
impl<'b, C> minicbor::Decode<'b, C> for Main {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        c: &mut C,
    ) -> core::result::Result<Main, minicbor::decode::Error> {
        let p = d.position();
        let mut instance_uuid: core::option::Option<Uuid> = None;
        let mut package_uuid: core::option::Option<Uuid> = None;
        let mut material_uuid: core::option::Option<Uuid> = None;
        let mut brand_uuid: core::option::Option<Uuid> = None;
        let mut gtin: core::option::Option<f32> = None;
        let mut brand_specific_instance_id: core::option::Option<String> = None;
        let mut brand_specific_package_id: core::option::Option<String> = None;
        let mut brand_specific_material_id: core::option::Option<String> = None;
        let mut material_class: core::option::Option<MaterialClass> = None;
        let mut material_type: core::option::Option<MaterialType> = None;
        let mut material_name: core::option::Option<String> = None;
        let mut material_abbreviation: core::option::Option<String> = None;
        let mut brand_name: core::option::Option<String> = None;
        let mut write_protection: core::option::Option<WriteProtection> = None;
        let mut manufactured_date: core::option::Option<Timestamp> = None;
        let mut country_of_origin: core::option::Option<String> = None;
        let mut expiration_date: core::option::Option<Timestamp> = None;
        let mut nominal_netto_full_weight: core::option::Option<f32> = None;
        let mut actual_netto_full_weight: core::option::Option<f32> = None;
        let mut nominal_full_length: core::option::Option<f32> = None;
        let mut actual_full_length: core::option::Option<f32> = None;
        let mut empty_container_weight: core::option::Option<f32> = None;
        let mut primary_color: core::option::Option<[u8; 4]> = None;
        let mut secondary_color_0: core::option::Option<[u8; 4]> = None;
        let mut secondary_color_1: core::option::Option<[u8; 4]> = None;
        let mut secondary_color_2: core::option::Option<[u8; 4]> = None;
        let mut secondary_color_3: core::option::Option<[u8; 4]> = None;
        let mut secondary_color_4: core::option::Option<[u8; 4]> = None;
        let mut transmission_distance: core::option::Option<f32> = None;
        let mut tags: core::option::Option<EnumArray<Tags, 71>> = None;
        let mut density: core::option::Option<f32> = None;
        let mut filament_diameter: core::option::Option<f32> = None;
        let mut shore_hardness_a: core::option::Option<u32> = None;
        let mut shore_hardness_d: core::option::Option<u32> = None;
        let mut min_nozzle_diameter: core::option::Option<f32> = None;
        let mut min_print_temperature: core::option::Option<u32> = None;
        let mut max_print_temperature: core::option::Option<u32> = None;
        let mut preheat_temperature: core::option::Option<u32> = None;
        let mut min_bed_temperature: core::option::Option<u32> = None;
        let mut max_bed_temperature: core::option::Option<u32> = None;
        let mut min_chamber_temperature: core::option::Option<u32> = None;
        let mut max_chamber_temperature: core::option::Option<u32> = None;
        let mut chamber_temperature: core::option::Option<u32> = None;
        let mut container_width: core::option::Option<u32> = None;
        let mut container_outer_diameter: core::option::Option<u32> = None;
        let mut container_inner_diameter: core::option::Option<u32> = None;
        let mut container_hole_diameter: core::option::Option<u32> = None;
        let mut viscosity_18c: core::option::Option<f32> = None;
        let mut viscosity_25c: core::option::Option<f32> = None;
        let mut viscosity_40c: core::option::Option<f32> = None;
        let mut viscosity_60c: core::option::Option<f32> = None;
        let mut container_volumetric_capacity: core::option::Option<f32> = None;
        let mut cure_wavelength: core::option::Option<u32> = None;
        if let Some(map_len) = d.map()? {
            for _ in 0..map_len {
                match d.i64()? {
                    0i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => instance_uuid = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    1i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => package_uuid = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    2i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => material_uuid = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    3i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => brand_uuid = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    4i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => gtin = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    5i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => brand_specific_instance_id = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    6i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => brand_specific_package_id = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    7i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => brand_specific_material_id = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    8i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => material_class = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    9i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => material_type = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    10i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => material_name = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    52i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => material_abbreviation = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    11i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => brand_name = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    13i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => write_protection = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    14i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => manufactured_date = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    55i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => country_of_origin = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    15i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => expiration_date = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    16i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => nominal_netto_full_weight = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    17i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => actual_netto_full_weight = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    53i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => nominal_full_length = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    54i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => actual_full_length = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    18i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => empty_container_weight = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    19i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => primary_color = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    20i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => secondary_color_0 = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    21i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => secondary_color_1 = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    22i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => secondary_color_2 = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    23i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => secondary_color_3 = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    24i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => secondary_color_4 = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    27i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => transmission_distance = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    28i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => tags = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    29i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => density = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    30i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => filament_diameter = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    31i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => shore_hardness_a = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    32i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => shore_hardness_d = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    33i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => min_nozzle_diameter = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    34i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => min_print_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    35i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => max_print_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    36i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => preheat_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    37i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => min_bed_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    38i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => max_bed_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    39i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => min_chamber_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    40i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => max_chamber_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    41i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => chamber_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    42i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => container_width = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    43i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => container_outer_diameter = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    44i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => container_inner_diameter = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    45i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => container_hole_diameter = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    46i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => viscosity_18c = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    47i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => viscosity_25c = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    48i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => viscosity_40c = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    49i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => viscosity_60c = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    50i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => container_volumetric_capacity = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    51i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => cure_wavelength = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    _ => d.skip()?,
                }
            }
        } else {
            while minicbor::data::Type::Break != d.datatype()? {
                match d.i64()? {
                    0i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => instance_uuid = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    1i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => package_uuid = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    2i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => material_uuid = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    3i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => brand_uuid = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    4i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => gtin = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    5i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => brand_specific_instance_id = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    6i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => brand_specific_package_id = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    7i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => brand_specific_material_id = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    8i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => material_class = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    9i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => material_type = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    10i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => material_name = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    52i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => material_abbreviation = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    11i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => brand_name = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    13i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => write_protection = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    14i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => manufactured_date = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    55i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => country_of_origin = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    15i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => expiration_date = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    16i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => nominal_netto_full_weight = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    17i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => actual_netto_full_weight = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    53i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => nominal_full_length = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    54i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => actual_full_length = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    18i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => empty_container_weight = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    19i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => primary_color = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    20i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => secondary_color_0 = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    21i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => secondary_color_1 = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    22i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => secondary_color_2 = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    23i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => secondary_color_3 = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    24i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => secondary_color_4 = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    27i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => transmission_distance = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    28i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => tags = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    29i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => density = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    30i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => filament_diameter = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    31i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => shore_hardness_a = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    32i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => shore_hardness_d = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    33i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => min_nozzle_diameter = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    34i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => min_print_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    35i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => max_print_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    36i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => preheat_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    37i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => min_bed_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    38i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => max_bed_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    39i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => min_chamber_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    40i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => max_chamber_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    41i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => chamber_temperature = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    42i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => container_width = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    43i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => container_outer_diameter = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    44i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => container_inner_diameter = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    45i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => container_hole_diameter = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    46i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => viscosity_18c = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    47i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => viscosity_25c = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    48i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => viscosity_40c = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    49i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => viscosity_60c = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    50i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => container_volumetric_capacity = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    51i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => cure_wavelength = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    _ => d.skip()?,
                }
            }
            d.skip()?
        }
        Ok(Main {
            instance_uuid: if let Some(v) = instance_uuid {
                v
            } else if let Some(z) = <Uuid as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::instance_uuid")
                    .at(p));
            },
            package_uuid: if let Some(v) = package_uuid {
                v
            } else if let Some(z) = <Uuid as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::package_uuid")
                    .at(p));
            },
            material_uuid: if let Some(v) = material_uuid {
                v
            } else if let Some(z) = <Uuid as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::material_uuid")
                    .at(p));
            },
            brand_uuid: if let Some(v) = brand_uuid {
                v
            } else if let Some(z) = <Uuid as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::brand_uuid")
                    .at(p));
            },
            gtin: if let Some(v) = gtin {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::gtin")
                    .at(p));
            },
            brand_specific_instance_id: if let Some(v) = brand_specific_instance_id {
                v
            } else if let Some(z) = <String as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::brand_specific_instance_id")
                    .at(p));
            },
            brand_specific_package_id: if let Some(v) = brand_specific_package_id {
                v
            } else if let Some(z) = <String as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::brand_specific_package_id")
                    .at(p));
            },
            brand_specific_material_id: if let Some(v) = brand_specific_material_id {
                v
            } else if let Some(z) = <String as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::brand_specific_material_id")
                    .at(p));
            },
            material_class: if let Some(v) = material_class {
                v
            } else if let Some(z) = <MaterialClass as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::material_class")
                    .at(p));
            },
            material_type: if let Some(v) = material_type {
                v
            } else if let Some(z) = <MaterialType as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::material_type")
                    .at(p));
            },
            material_name: if let Some(v) = material_name {
                v
            } else if let Some(z) = <String as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::material_name")
                    .at(p));
            },
            material_abbreviation: if let Some(v) = material_abbreviation {
                v
            } else if let Some(z) = <String as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::material_abbreviation")
                    .at(p));
            },
            brand_name: if let Some(v) = brand_name {
                v
            } else if let Some(z) = <String as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::brand_name")
                    .at(p));
            },
            write_protection: if let Some(v) = write_protection {
                v
            } else if let Some(z) = <WriteProtection as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::write_protection")
                    .at(p));
            },
            manufactured_date: if let Some(v) = manufactured_date {
                v
            } else if let Some(z) = <Timestamp as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::manufactured_date")
                    .at(p));
            },
            country_of_origin: if let Some(v) = country_of_origin {
                v
            } else if let Some(z) = <String as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::country_of_origin")
                    .at(p));
            },
            expiration_date: if let Some(v) = expiration_date {
                v
            } else if let Some(z) = <Timestamp as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::expiration_date")
                    .at(p));
            },
            nominal_netto_full_weight: if let Some(v) = nominal_netto_full_weight {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::nominal_netto_full_weight")
                    .at(p));
            },
            actual_netto_full_weight: if let Some(v) = actual_netto_full_weight {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::actual_netto_full_weight")
                    .at(p));
            },
            nominal_full_length: if let Some(v) = nominal_full_length {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::nominal_full_length")
                    .at(p));
            },
            actual_full_length: if let Some(v) = actual_full_length {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::actual_full_length")
                    .at(p));
            },
            empty_container_weight: if let Some(v) = empty_container_weight {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::empty_container_weight")
                    .at(p));
            },
            primary_color: if let Some(v) = primary_color {
                v
            } else if let Some(z) = <[u8; 4] as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::primary_color")
                    .at(p));
            },
            secondary_color_0: if let Some(v) = secondary_color_0 {
                v
            } else if let Some(z) = <[u8; 4] as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::secondary_color_0")
                    .at(p));
            },
            secondary_color_1: if let Some(v) = secondary_color_1 {
                v
            } else if let Some(z) = <[u8; 4] as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::secondary_color_1")
                    .at(p));
            },
            secondary_color_2: if let Some(v) = secondary_color_2 {
                v
            } else if let Some(z) = <[u8; 4] as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::secondary_color_2")
                    .at(p));
            },
            secondary_color_3: if let Some(v) = secondary_color_3 {
                v
            } else if let Some(z) = <[u8; 4] as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::secondary_color_3")
                    .at(p));
            },
            secondary_color_4: if let Some(v) = secondary_color_4 {
                v
            } else if let Some(z) = <[u8; 4] as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::secondary_color_4")
                    .at(p));
            },
            transmission_distance: if let Some(v) = transmission_distance {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::transmission_distance")
                    .at(p));
            },
            tags: if let Some(v) = tags {
                v
            } else if let Some(z) = <EnumArray<Tags, 71> as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::tags")
                    .at(p));
            },
            density: if let Some(v) = density {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::density")
                    .at(p));
            },
            filament_diameter: if let Some(v) = filament_diameter {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::filament_diameter")
                    .at(p));
            },
            shore_hardness_a: if let Some(v) = shore_hardness_a {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::shore_hardness_a")
                    .at(p));
            },
            shore_hardness_d: if let Some(v) = shore_hardness_d {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::shore_hardness_d")
                    .at(p));
            },
            min_nozzle_diameter: if let Some(v) = min_nozzle_diameter {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::min_nozzle_diameter")
                    .at(p));
            },
            min_print_temperature: if let Some(v) = min_print_temperature {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::min_print_temperature")
                    .at(p));
            },
            max_print_temperature: if let Some(v) = max_print_temperature {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::max_print_temperature")
                    .at(p));
            },
            preheat_temperature: if let Some(v) = preheat_temperature {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::preheat_temperature")
                    .at(p));
            },
            min_bed_temperature: if let Some(v) = min_bed_temperature {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::min_bed_temperature")
                    .at(p));
            },
            max_bed_temperature: if let Some(v) = max_bed_temperature {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::max_bed_temperature")
                    .at(p));
            },
            min_chamber_temperature: if let Some(v) = min_chamber_temperature {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::min_chamber_temperature")
                    .at(p));
            },
            max_chamber_temperature: if let Some(v) = max_chamber_temperature {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::max_chamber_temperature")
                    .at(p));
            },
            chamber_temperature: if let Some(v) = chamber_temperature {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::chamber_temperature")
                    .at(p));
            },
            container_width: if let Some(v) = container_width {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::container_width")
                    .at(p));
            },
            container_outer_diameter: if let Some(v) = container_outer_diameter {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::container_outer_diameter")
                    .at(p));
            },
            container_inner_diameter: if let Some(v) = container_inner_diameter {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::container_inner_diameter")
                    .at(p));
            },
            container_hole_diameter: if let Some(v) = container_hole_diameter {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::container_hole_diameter")
                    .at(p));
            },
            viscosity_18c: if let Some(v) = viscosity_18c {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::viscosity_18c")
                    .at(p));
            },
            viscosity_25c: if let Some(v) = viscosity_25c {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::viscosity_25c")
                    .at(p));
            },
            viscosity_40c: if let Some(v) = viscosity_40c {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::viscosity_40c")
                    .at(p));
            },
            viscosity_60c: if let Some(v) = viscosity_60c {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::viscosity_60c")
                    .at(p));
            },
            container_volumetric_capacity: if let Some(v) = container_volumetric_capacity {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::container_volumetric_capacity")
                    .at(p));
            },
            cure_wavelength: if let Some(v) = cure_wavelength {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Main::cure_wavelength")
                    .at(p));
            },
        })
    }
}
#[derive(Debug, Clone)]
///Dynamic data, typically usage tracking.
pub struct Aux {
    /**Amount of material that was used up from the container.

    `remaining_weight` = `instance_netto_full_weight` - `consumed_weight`*/
    pub consumed_weight: f32,
    /**Workgroup identifier, used for detecting first usage of the material.

    See the _write protection_ section.*/
    pub workgroup: String,
    /**Determines semantics of the fields in the general purpose key range.

    MUST be filled if any of the general purpose keys is used.
    See _Vendor-specific fields_.*/
    pub general_purpose_range_user: String,
    /**Timestamp when the resin was last stirred.

    Resins that have not been used for some time should be stirred before printing.*/
    pub last_stir_time: Timestamp,
}
impl<'b, C> minicbor::Decode<'b, C> for Aux {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        c: &mut C,
    ) -> core::result::Result<Aux, minicbor::decode::Error> {
        let p = d.position();
        let mut consumed_weight: core::option::Option<f32> = None;
        let mut workgroup: core::option::Option<String> = None;
        let mut general_purpose_range_user: core::option::Option<String> = None;
        let mut last_stir_time: core::option::Option<Timestamp> = None;
        if let Some(map_len) = d.map()? {
            for _ in 0..map_len {
                match d.i64()? {
                    0i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => consumed_weight = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    1i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => workgroup = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    2i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => general_purpose_range_user = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    3i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => last_stir_time = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    _ => d.skip()?,
                }
            }
        } else {
            while minicbor::data::Type::Break != d.datatype()? {
                match d.i64()? {
                    0i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => consumed_weight = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    1i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => workgroup = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    2i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => general_purpose_range_user = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    3i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => last_stir_time = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    _ => d.skip()?,
                }
            }
            d.skip()?
        }
        Ok(Aux {
            consumed_weight: if let Some(v) = consumed_weight {
                v
            } else if let Some(z) = <f32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Aux::consumed_weight")
                    .at(p));
            },
            workgroup: if let Some(v) = workgroup {
                v
            } else if let Some(z) = <String as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Aux::workgroup")
                    .at(p));
            },
            general_purpose_range_user: if let Some(v) = general_purpose_range_user {
                v
            } else if let Some(z) = <String as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Aux::general_purpose_range_user")
                    .at(p));
            },
            last_stir_time: if let Some(v) = last_stir_time {
                v
            } else if let Some(z) = <Timestamp as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Aux::last_stir_time")
                    .at(p));
            },
        })
    }
}
#[derive(Debug, Clone)]
///Offsets and sizes within the `NDEF` payload.
pub struct Meta {
    /**Offset of the main region, relative to the NDEF payload start.

    If not specified, the main region immediately follows the meta section.*/
    pub main_region_offset: u32,
    /**Allocation size of the main region.

    If not specified, the region spans till the next region or payload end.*/
    pub main_region_size: u32,
    /**Offset of the auxiliary region, relative to the NDEF payload start.

    Omitting this field means that the auxiliary region is not present.*/
    pub aux_region_offset: u32,
    /**Allocation size of the auxiliary region.

    If not specified, the region (if present) spans till the next region or payload end.*/
    pub aux_region_size: u32,
}
impl<'b, C> minicbor::Decode<'b, C> for Meta {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        c: &mut C,
    ) -> core::result::Result<Meta, minicbor::decode::Error> {
        let p = d.position();
        let mut main_region_offset: core::option::Option<u32> = None;
        let mut main_region_size: core::option::Option<u32> = None;
        let mut aux_region_offset: core::option::Option<u32> = None;
        let mut aux_region_size: core::option::Option<u32> = None;
        if let Some(map_len) = d.map()? {
            for _ in 0..map_len {
                match d.i64()? {
                    0i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => main_region_offset = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    1i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => main_region_size = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    2i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => aux_region_offset = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    3i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => aux_region_size = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    _ => d.skip()?,
                }
            }
        } else {
            while minicbor::data::Type::Break != d.datatype()? {
                match d.i64()? {
                    0i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => main_region_offset = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    1i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => main_region_size = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    2i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => aux_region_offset = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    3i64 => match minicbor::Decode::decode(d, c) {
                        Ok(v) => aux_region_size = Some(v),
                        Err(e) if e.is_unknown_variant() => d.skip()?,
                        Err(e) => return Err(e),
                    },
                    _ => d.skip()?,
                }
            }
            d.skip()?
        }
        Ok(Meta {
            main_region_offset: if let Some(v) = main_region_offset {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Meta::main_region_offset")
                    .at(p));
            },
            main_region_size: if let Some(v) = main_region_size {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Meta::main_region_size")
                    .at(p));
            },
            aux_region_offset: if let Some(v) = aux_region_offset {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Meta::aux_region_offset")
                    .at(p));
            },
            aux_region_size: if let Some(v) = aux_region_size {
                v
            } else if let Some(z) = <u32 as minicbor::Decode<C>>::nil() {
                z
            } else {
                return Err(minicbor::decode::Error::missing_value(2)
                    .with_message("Meta::aux_region_size")
                    .at(p));
            },
        })
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
///Contains material information that does not change during the package instance lifetime.
pub enum MaterialClass {
    #[doc = "**Filament**\nFilament"]
    FFF = 0u16,
    #[doc = "**Resin**\nResin"]
    SLA = 1u16,
}
impl TryFrom<u16> for MaterialClass {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0u16 => Ok(Self::FFF),
            1u16 => Ok(Self::SLA),
            other => Err(Error::InvalidEnumDiscriminant(other)),
        }
    }
}
impl TryFrom<u8> for MaterialClass {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > u16::MAX as u8 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<u32> for MaterialClass {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > u16::MAX as u32 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<u64> for MaterialClass {
    type Error = Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > u16::MAX as u64 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<usize> for MaterialClass {
    type Error = Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > u16::MAX as usize {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i8> for MaterialClass {
    type Error = Error;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i8 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i16> for MaterialClass {
    type Error = Error;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i16 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i32> for MaterialClass {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i32 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i64> for MaterialClass {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i64 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<isize> for MaterialClass {
    type Error = Error;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as isize {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl<'b, C> minicbor::Decode<'b, C> for MaterialClass {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<MaterialClass, minicbor::decode::Error> {
        let pos = d.position();
        let n = d.i64()?;
        MaterialClass::try_from(n).map_err(|_| minicbor::decode::Error::unknown_variant(n).at(pos))
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
///Contains material information that does not change during the package instance lifetime.
pub enum MaterialType {
    #[doc = "**Polylactic Acid**\nEasy-to-print, biodegradable material. Ideal for beginners, prototypes, and models."]
    PLA = 0u16,
    #[doc = "**Polyethylene Terephthalate Glycol**\nDurable, strong, and temperature-resistant. Great for mechanical parts and functional prints."]
    PETG = 1u16,
    #[doc = "**Thermoplastic Polyurethane**\nA flexible, rubber-like material. Used for phone cases, vibration dampeners, and other soft parts."]
    TPU = 2u16,
    #[doc = "**Acrylonitrile Butadiene Styrene**\nStrong, durable, and heat-resistant plastic. Used for functional parts like car interiors and LEGOs. Requires a heated bed and enclosure."]
    ABS = 3u16,
    #[doc = "**Acrylonitrile Styrene Acrylate**\nSimilar to ABS but with high UV and weather resistance, making it perfect for outdoor applications."]
    ASA = 4u16,
    #[doc = "**Polycarbonate**\nExtremely strong, impact-resistant, and heat-resistant. Used for demanding engineering applications."]
    PC = 5u16,
    #[doc = "**Polycyclohexylenedimethylene Terephthalate Glycol**\nA tougher alternative to PETG with higher impact and chemical resistance."]
    PCTG = 6u16,
    #[doc = "**Polypropylene**\nLightweight, chemically resistant, and flexible. Used for creating living hinges and durable containers."]
    PP = 7u16,
    #[doc = "**Polyamide 6**\nA type of Nylon that is tough and wear-resistant but absorbs more moisture than other nylons."]
    PA6 = 8u16,
    #[doc = "**Polyamide 11**\nA flexible, bio-based Nylon with low moisture absorption and good chemical resistance."]
    PA11 = 9u16,
    #[doc = "**Polyamide 12**\nThe most common Nylon for 3D printing. Strong, tough, with low moisture absorption. Great for functional parts."]
    PA12 = 10u16,
    #[doc = "**Polyamide 66**\nA stiffer and more heat-resistant Nylon compared to PA6, used for durable mechanical parts."]
    PA66 = 11u16,
    #[doc = "**Copolyester**\nA family of strong and dimensionally stable materials (including PETG) known for chemical resistance."]
    CPE = 12u16,
    #[doc = "**Thermoplastic Elastomer**\nA general class of soft, rubbery materials. Softer and more flexible than TPU."]
    TPE = 13u16,
    #[doc = "**High Impact Polystyrene**\nA lightweight material often used as a dissolvable support material for ABS prints (dissolves in Limonene)."]
    HIPS = 14u16,
    #[doc = "**Polyhydroxyalkanoate**\nA biodegradable material similar to PLA but with better toughness and flexibility."]
    PHA = 15u16,
    #[doc = "**Polyethylene Terephthalate**\nThe same plastic used in water bottles. Strong and food-safe, but less common for printing than PETG."]
    PET = 16u16,
    #[doc = "**Polyetherimide**\nA high-performance material (also known as Ultem) with excellent thermal and mechanical properties."]
    PEI = 17u16,
    #[doc = "**Polybutylene Terephthalate**\nAn engineering polymer with good heat resistance and electrical insulation properties."]
    PBT = 18u16,
    #[doc = "**Polyvinyl Butyral**\nEasy to print and can be chemically smoothed with isopropyl alcohol for a glossy finish."]
    PVB = 19u16,
    #[doc = "**Polyvinyl Alcohol**\nA water-soluble filament used exclusively as a support material for complex prints."]
    PVA = 20u16,
    #[doc = "**Polyetherketoneketone**\nAn ultra-high-performance polymer with exceptional heat, chemical, and mechanical properties for industrial use."]
    PEKK = 21u16,
    #[doc = "**Polyether Ether Ketone**\nAn ultra-high-performance polymer with exceptional mechanical, thermal, and chemical resistance. Used in demanding aerospace, medical, and industrial applications."]
    PEEK = 22u16,
    #[doc = "**Butenediol Vinyl Alcohol Copolymer**\nA water-soluble support material that often dissolves faster and is easier to print than PVA."]
    BVOH = 23u16,
    #[doc = "**Thermoplastic Copolyester**\nA flexible, TPE-like material with good thermal and chemical resistance."]
    TPC = 24u16,
    #[doc = "**Polyphenylene Sulfide**\nA high-performance polymer known for its thermal stability and chemical resistance, often used in automotive and electronics."]
    PPS = 25u16,
    #[doc = "**Polyphenylsulfone**\nA high-performance material with excellent heat and chemical resistance, often used in medical applications."]
    PPSU = 26u16,
    #[doc = "**Polyvinyl Chloride**\nStrong and durable but rarely used in 3D printing due to the release of toxic fumes."]
    PVC = 27u16,
    #[doc = "**Polyether Block Amide**\nA flexible and lightweight TPE known for its excellent energy return, used in sports equipment."]
    PEBA = 28u16,
    #[doc = "**Polyvinylidene Fluoride**\nHigh-performance polymer with excellent resistance to chemicals and UV light."]
    PVDF = 29u16,
    #[doc = "**Polyphthalamide**\nA high-performance Nylon with superior strength, stiffness, and heat resistance compared to standard Nylons."]
    PPA = 30u16,
    #[doc = "**Polycaprolactone**\nA biodegradable polyester with a very low melting point (~60 °C), allowing it to be reshaped by hand in hot water."]
    PCL = 31u16,
    #[doc = "**Polyethersulfone**\nA high-temperature, amorphous polymer with good chemical and hydrolytic stability."]
    PES = 32u16,
    #[doc = "**Polymethyl Methacrylate**\nA rigid, transparent material also known as acrylic. Offers good optical clarity."]
    PMMA = 33u16,
    #[doc = "**Polyoxymethylene**\nA low-friction, rigid material also known as Delrin. Excellent for gears, bearings, and moving parts."]
    POM = 34u16,
    #[doc = "**Polyphenylene Ether**\nAn engineering thermoplastic with good temperature resistance and dimensional stability, often used in blends."]
    PPE = 35u16,
    #[doc = "**Polystyrene**\nA lightweight and brittle material. Not commonly used in its pure form for 3D printing."]
    PS = 36u16,
    #[doc = "**Polysulfone**\nA high-temperature material with good thermal stability and chemical resistance."]
    PSU = 37u16,
    #[doc = "**Thermoplastic Polyimide**\nAn ultra-high-performance polymer with one of the highest glass transition temperatures and excellent thermal stability."]
    TPI = 38u16,
    #[doc = "**Styrene-Butadiene-Styrene**\nA flexible, rubber-like material (a type of TPE) known for good durability. It is relatively easy to print for a flexible filament."]
    SBS = 39u16,
}
impl TryFrom<u16> for MaterialType {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0u16 => Ok(Self::PLA),
            1u16 => Ok(Self::PETG),
            2u16 => Ok(Self::TPU),
            3u16 => Ok(Self::ABS),
            4u16 => Ok(Self::ASA),
            5u16 => Ok(Self::PC),
            6u16 => Ok(Self::PCTG),
            7u16 => Ok(Self::PP),
            8u16 => Ok(Self::PA6),
            9u16 => Ok(Self::PA11),
            10u16 => Ok(Self::PA12),
            11u16 => Ok(Self::PA66),
            12u16 => Ok(Self::CPE),
            13u16 => Ok(Self::TPE),
            14u16 => Ok(Self::HIPS),
            15u16 => Ok(Self::PHA),
            16u16 => Ok(Self::PET),
            17u16 => Ok(Self::PEI),
            18u16 => Ok(Self::PBT),
            19u16 => Ok(Self::PVB),
            20u16 => Ok(Self::PVA),
            21u16 => Ok(Self::PEKK),
            22u16 => Ok(Self::PEEK),
            23u16 => Ok(Self::BVOH),
            24u16 => Ok(Self::TPC),
            25u16 => Ok(Self::PPS),
            26u16 => Ok(Self::PPSU),
            27u16 => Ok(Self::PVC),
            28u16 => Ok(Self::PEBA),
            29u16 => Ok(Self::PVDF),
            30u16 => Ok(Self::PPA),
            31u16 => Ok(Self::PCL),
            32u16 => Ok(Self::PES),
            33u16 => Ok(Self::PMMA),
            34u16 => Ok(Self::POM),
            35u16 => Ok(Self::PPE),
            36u16 => Ok(Self::PS),
            37u16 => Ok(Self::PSU),
            38u16 => Ok(Self::TPI),
            39u16 => Ok(Self::SBS),
            other => Err(Error::InvalidEnumDiscriminant(other)),
        }
    }
}
impl TryFrom<u8> for MaterialType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > u16::MAX as u8 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<u32> for MaterialType {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > u16::MAX as u32 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<u64> for MaterialType {
    type Error = Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > u16::MAX as u64 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<usize> for MaterialType {
    type Error = Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > u16::MAX as usize {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i8> for MaterialType {
    type Error = Error;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i8 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i16> for MaterialType {
    type Error = Error;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i16 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i32> for MaterialType {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i32 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i64> for MaterialType {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i64 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<isize> for MaterialType {
    type Error = Error;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as isize {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl<'b, C> minicbor::Decode<'b, C> for MaterialType {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<MaterialType, minicbor::decode::Error> {
        let pos = d.position();
        let n = d.i64()?;
        MaterialType::try_from(n).map_err(|_| minicbor::decode::Error::unknown_variant(n).at(pos))
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
///Contains material information that does not change during the package instance lifetime.
pub enum WriteProtection {
    ///The tag is not write protected.
    No = 0u16,
    ///The tag is irreversibly protected against writing.
    Irreversible = 1u16,
    ///The tag is write-protected using the `PROTECT PAGE` command (SLIX2-specific) and is unlockable with a password that is located somewhere on the container.
    ProtectPageUnlockable = 2u16,
}
impl TryFrom<u16> for WriteProtection {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0u16 => Ok(Self::No),
            1u16 => Ok(Self::Irreversible),
            2u16 => Ok(Self::ProtectPageUnlockable),
            other => Err(Error::InvalidEnumDiscriminant(other)),
        }
    }
}
impl TryFrom<u8> for WriteProtection {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > u16::MAX as u8 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<u32> for WriteProtection {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > u16::MAX as u32 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<u64> for WriteProtection {
    type Error = Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > u16::MAX as u64 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<usize> for WriteProtection {
    type Error = Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > u16::MAX as usize {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i8> for WriteProtection {
    type Error = Error;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i8 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i16> for WriteProtection {
    type Error = Error;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i16 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i32> for WriteProtection {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i32 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i64> for WriteProtection {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i64 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<isize> for WriteProtection {
    type Error = Error;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as isize {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl<'b, C> minicbor::Decode<'b, C> for WriteProtection {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<WriteProtection, minicbor::decode::Error> {
        let pos = d.position();
        let n = d.i64()?;
        WriteProtection::try_from(n)
            .map_err(|_| minicbor::decode::Error::unknown_variant(n).at(pos))
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
///Contains material information that does not change during the package instance lifetime.
pub enum Tags {
    #[doc = "**Filtration recommended**\nReleases a higher concentration of unsafe particles/fumes during printing so a HEPA and carbon filter is strongly recommended."]
    FiltrationRecommended = 0u16,
    #[doc = "**Biocompatible**\nCertified biocompatibility (does not cause harmful effects when in contact with the body)."]
    Biocompatible = 1u16,
    #[doc = "**Home compostable**\nDecomposes into natural elements in a home compost system at ambient temperatures."]
    HomeCompostable = 61u16,
    #[doc = "**Industrially compostable**\nDecomposes into natural elements under specific temperature and microbial conditions in commercial composting facilities."]
    IndustriallyCompostable = 62u16,
    #[doc = "**Bio-based**\nPredominantly made from renewable biological resources, like plants."]
    BioBased = 63u16,
    #[doc = "**Antibacterial**\nHas antibacterial properties."]
    Antibacterial = 2u16,
    #[doc = "**Air filtering**\nHas air filtering properties (absorbs/filters harmful compounds/particles from the air)."]
    AirFiltering = 3u16,
    #[doc = "**Abrasive**\nThe material is abrasive and requires an abrasive-resistant nozzle."]
    Abrasive = 4u16,
    #[doc = "**Foaming**\nThe material increases its volume during extrusion."]
    Foaming = 5u16,
    #[doc = "**Castable**\nThe material is suitable to be used as a sacrificial pattern for investment casting.\n\nIt can be cleanly removed from the mold (typically burned out or melted away), leaving minimal residue (for example ashes).\nThis does NOT mean that the material is used for the mold or the final cast itself, only the investment pattern."]
    Castable = 67u16,
    #[doc = "**Self-extinguishing**\nThe material is self-extinguishing. This does not mean that the material is not flammable, just that burning it implies more energy than it produces.\n\nMeets at least UL 94 HB."]
    SelfExtinguishing = 6u16,
    #[doc = "**Paramagnetic**\nThe material has paramagnetic properties, meaning that it is (weakly) attracted to magnets."]
    Paramagnetic = 7u16,
    #[doc = "**Radiation shielding**\nHas radiation shielding properties."]
    RadiationShielding = 8u16,
    #[doc = "**High temperature**\nThe material softens at higher temperatures than what is common for the material type, while keeping similar printing temperatures.\n\nCan be used for HTPLA filament while keeping the PLA material type.\nThis does NOT indicate increase resistance to flame/burning.\n**Note:** If the material type would be 'HTPLA', adding this tag would mean 'high-temperature variant of a high-temperature PLA'"]
    HighTemperature = 9u16,
    #[doc = "**ESD safe**\nThe material is static dissipative - prevents electrostatic charge buildup by allowing gradual dissipation of the charge.\n\nUseful for protecting sensitive electronic components.\nSheet resistance R >= 1e5 Ω/□ && R < 1e12 Ω/□ or volumetric resistivity ρ >= 1e4 Ω⋅cm && ρ < 1e11 Ω⋅cm.\nThe tag does NOT cover the \"anti-static\" materials (that have higher resistances)."]
    EsdSafe = 10u16,
    #[doc = "**Conductive**\nThe material can conduct electricity.\n\nThis does NOT mean that it the material is a good conductor, such as metals.\nCommon \"conductive\" material have resistances in the range of kiloohms on 10 cm of filament.\nSheet resistance R < 1e5 Ω/□ or volumetric resistivity ρ < 1e4 Ω⋅cm."]
    Conductive = 11u16,
    #[doc = "**EMI shielding**\nThe material can be effectively used for shielding against electromagnetic interference.\n\nSheet resistance R < 1 Ω/□ or volumetric resistivity ρ < 1e-2 Ω⋅cm.\n\n**Implies:** [`Conductive`][ConfigNfcv::Conductive]"]
    EmiShielding = 70u16,
    #[doc = "**Blend**\nThe material is a blend of multiple polymers or a base polymer with significant additives that alter its properties and may require a specific print profile."]
    Blend = 12u16,
    #[doc = "**Water soluble**\nCan be dissolved in water."]
    WaterSoluble = 13u16,
    #[doc = "**IPA soluble**\nCan be dissolved in IPA (isopropyl alcohol)."]
    IpaSoluble = 14u16,
    #[doc = "**Limonene soluble**\nCan be dissolved in limonene."]
    LimoneneSoluble = 15u16,
    #[doc = "**Low outgassing**\nReleases only minimal gas (and vapor) when placed in a vacuum."]
    LowOutgassing = 64u16,
    #[doc = "**Matte**\nProduces matte, non-shiny surface (very low specular reflection coefficient)."]
    Matte = 16u16,
    #[doc = "**Silk**\nProduces smooth, shiny/glossy surface (higher specular reflection coefficient)."]
    Silk = 17u16,
    #[doc = "**Translucent**\nNot fully opaque – [HueForge TD](https://shop.thehueforge.com/blogs/news/what-is-hueforge) >= `X` (exact `X` will be determined later).\n\nThe material with this tag can possibly disperse light, meaning that while the light goes through it, the image is \"blurred\" and one does not see clearly what's on the other side. See the `transparent` tag."]
    Translucent = 19u16,
    #[doc = "**Transparent**\nNot fully opaque, does not disperse light.\n\nUnder correct printing conditions, can be printed with a see-through glass-like transparency.\n\n**Implies:** [`Translucent`][ConfigNfcv::Translucent]"]
    Transparent = 20u16,
    #[doc = "**Without pigments**\nThe material is of its \"natural\" color, no pigments were added.\n\n**Hints:** [`Translucent`][ConfigNfcv::Translucent]"]
    WithoutPigments = 65u16,
    #[doc = "**Iridescent**\nSame as mystic.\n\nChanges color based on the viewing angle.\nSee https://en.wikipedia.org/wiki/Iridescence"]
    Iridescent = 21u16,
    #[doc = "**Pearlescent**\nSpecial case of iridescent where the reflected light is mostly white.\n\nSee https://en.wikipedia.org/wiki/Iridescence#Pearlescence\n\n**Implies:** [`Iridescent`][ConfigNfcv::Iridescent]"]
    Pearlescent = 22u16,
    #[doc = "**Glitter**\nContains coarse glitter particles, causing a shimmering effect.\n\nSimilar to iridescent/pearlescent, but the individual particles causing the effect are larger, visible with the naked eye."]
    Glitter = 23u16,
    #[doc = "**Glow in the dark**\nGlows in the dark (phosphorescent).\n\nThe glow color doesn't necessarily match the base material color (`illuminescent_color_change`).\nThe different glow color can be specified as secondary color of the material."]
    GlowInTheDark = 24u16,
    #[doc = "**Neon**\nNeon color/glows under UV light (fluorescent).\n\nThe glow color doesn't necessarily match the base material color (`illuminescent_color_change`).\nThe different glow color can be specified as secondary color of the material."]
    Neon = 25u16,
    #[doc = "**Illuminescent color change**\nThe glow color (caused by illuminiscence) is different to the material base color.\n\nFor example the material is blue, but glows green in the dark or under the UV light.\nThe glow color can be specified as a secondary color of the material."]
    IlluminescentColorChange = 26u16,
    #[doc = "**Temperature color change**\nChanges color based on the temperature."]
    TemperatureColorChange = 27u16,
    #[doc = "**Gradual color change**\nTransitions between colors as the filament is extruded.\n\nDoes not necessary mean that the filament must go through the rainbow colors, gradual color change between two colors is enough to qualify."]
    GradualColorChange = 28u16,
    #[doc = "**Coextruded**\nCo-extruded from multiple colors. The colors are all present at any cross-section of the filament.\n\nDo not confuse with `gradual_color_change`.\nDoes not have a primary color, number of colors can be derived from the defined secondary colors."]
    Coextruded = 29u16,
    #[doc = "**Contains carbon**\nContains carbon."]
    ContainsCarbon = 30u16,
    #[doc = "**Contains carbon fiber**\nContains carbon fibers.\n\n**Implies:** [`ContainsCarbon`][ConfigNfcv::ContainsCarbon]"]
    ContainsCarbonFiber = 31u16,
    #[doc = "**Contains carbon nano tubes**\nContains carbon nano tubes.\n\n**Implies:** [`ContainsCarbon`][ConfigNfcv::ContainsCarbon]"]
    ContainsCarbonNanoTubes = 32u16,
    #[doc = "**Contains glass**\nContains glass."]
    ContainsGlass = 33u16,
    #[doc = "**Contains glass fiber**\nContains glass fibers.\n\n**Implies:** [`ContainsGlass`][ConfigNfcv::ContainsGlass]"]
    ContainsGlassFiber = 34u16,
    #[doc = "**Contains Kevlar**\nContains kevlar (aramid)."]
    ContainsKevlar = 35u16,
    #[doc = "**Contains PTFE**\nContains polytetrafluoroethylene (PTFE)."]
    ContainsPtfe = 68u16,
    #[doc = "**Contains stone**\nContains stone.\n\n**Hints:** [`Abrasive`][ConfigNfcv::Abrasive]"]
    ContainsStone = 36u16,
    #[doc = "**Contains magnetite**\nContains magnetite.\n\n**Hints:** [`Abrasive`][ConfigNfcv::Abrasive]"]
    ContainsMagnetite = 37u16,
    #[doc = "**Contains organic material**\nContains organic material."]
    ContainsOrganicMaterial = 38u16,
    #[doc = "**Contains cork**\nContains cork.\n\n**Implies:** [`ContainsOrganicMaterial`][ConfigNfcv::ContainsOrganicMaterial]"]
    ContainsCork = 39u16,
    #[doc = "**Contains wax**\nContains wax.\n\n**Implies:** [`ContainsOrganicMaterial`][ConfigNfcv::ContainsOrganicMaterial]"]
    ContainsWax = 40u16,
    #[doc = "**Contains wood**\nContains wood.\n\n**Implies:** [`ContainsOrganicMaterial`][ConfigNfcv::ContainsOrganicMaterial]"]
    ContainsWood = 41u16,
    #[doc = "**Contains algae**\nContains algae.\n\n**Implies:** [`ContainsOrganicMaterial`][ConfigNfcv::ContainsOrganicMaterial]"]
    ContainsAlgae = 66u16,
    #[doc = "**Contains bamboo**\nContains bamboo.\n\n**Implies:** [`ContainsWood`][ConfigNfcv::ContainsWood]"]
    ContainsBamboo = 42u16,
    #[doc = "**Contains pine**\nContains pine.\n\n**Implies:** [`ContainsWood`][ConfigNfcv::ContainsWood]"]
    ContainsPine = 43u16,
    #[doc = "**Contains ceramic**\nContains ceramic.\n\n**Hints:** [`Abrasive`][ConfigNfcv::Abrasive]"]
    ContainsCeramic = 44u16,
    #[doc = "**Contains boron carbide**\nContains boron carbide (useful for radiation shielding).\n\n**Hints:** [`RadiationShielding`][ConfigNfcv::RadiationShielding]\n**Implies:** [`ContainsCeramic`][ConfigNfcv::ContainsCeramic]"]
    ContainsBoronCarbide = 45u16,
    #[doc = "**Contains metal**\nContains metal. Specific type of metal contained can be expressed by an other tag.\n\n**Hints:** [`Abrasive`][ConfigNfcv::Abrasive]"]
    ContainsMetal = 46u16,
    #[doc = "**Contains bronze**\nContains bronze.\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsBronze = 47u16,
    #[doc = "**Contains iron**\nContains iron.\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsIron = 48u16,
    #[doc = "**Contains steel**\nContains steel.\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsSteel = 49u16,
    #[doc = "**Contains silver**\nContains silver (useful for antibacterial properties).\n\n**Hints:** [`Antibacterial`][ConfigNfcv::Antibacterial]\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsSilver = 50u16,
    #[doc = "**Contains copper**\nContains copper.\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsCopper = 51u16,
    #[doc = "**Contains aluminium**\nContains aluminium.\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsAluminium = 52u16,
    #[doc = "**Contains brass**\nContains brass.\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsBrass = 53u16,
    #[doc = "**Contains tungsten**\nContains Tungsten (useful for radiation shielding).\n\n**Hints:** [`RadiationShielding`][ConfigNfcv::RadiationShielding]\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsTungsten = 54u16,
    #[doc = "**Imitates wood**\nImitates wood."]
    ImitatesWood = 55u16,
    #[doc = "**Imitates metal**\nImitates metal."]
    ImitatesMetal = 56u16,
    #[doc = "**Imitates marble**\nImitates marble."]
    ImitatesMarble = 57u16,
    #[doc = "**Imitates stone**\nImitates stone."]
    ImitatesStone = 58u16,
    #[doc = "**Lithophane**\nSpecifically designed for lithophaning."]
    Lithophane = 59u16,
    #[doc = "**Recycled**\nPart of the material is recycled."]
    Recycled = 60u16,
    #[doc = "**Limited edition**\nThe material is a limited edition run."]
    LimitedEdition = 69u16,
}
impl TryFrom<u16> for Tags {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0u16 => Ok(Self::FiltrationRecommended),
            1u16 => Ok(Self::Biocompatible),
            61u16 => Ok(Self::HomeCompostable),
            62u16 => Ok(Self::IndustriallyCompostable),
            63u16 => Ok(Self::BioBased),
            2u16 => Ok(Self::Antibacterial),
            3u16 => Ok(Self::AirFiltering),
            4u16 => Ok(Self::Abrasive),
            5u16 => Ok(Self::Foaming),
            67u16 => Ok(Self::Castable),
            6u16 => Ok(Self::SelfExtinguishing),
            7u16 => Ok(Self::Paramagnetic),
            8u16 => Ok(Self::RadiationShielding),
            9u16 => Ok(Self::HighTemperature),
            10u16 => Ok(Self::EsdSafe),
            11u16 => Ok(Self::Conductive),
            70u16 => Ok(Self::EmiShielding),
            12u16 => Ok(Self::Blend),
            13u16 => Ok(Self::WaterSoluble),
            14u16 => Ok(Self::IpaSoluble),
            15u16 => Ok(Self::LimoneneSoluble),
            64u16 => Ok(Self::LowOutgassing),
            16u16 => Ok(Self::Matte),
            17u16 => Ok(Self::Silk),
            19u16 => Ok(Self::Translucent),
            20u16 => Ok(Self::Transparent),
            65u16 => Ok(Self::WithoutPigments),
            21u16 => Ok(Self::Iridescent),
            22u16 => Ok(Self::Pearlescent),
            23u16 => Ok(Self::Glitter),
            24u16 => Ok(Self::GlowInTheDark),
            25u16 => Ok(Self::Neon),
            26u16 => Ok(Self::IlluminescentColorChange),
            27u16 => Ok(Self::TemperatureColorChange),
            28u16 => Ok(Self::GradualColorChange),
            29u16 => Ok(Self::Coextruded),
            30u16 => Ok(Self::ContainsCarbon),
            31u16 => Ok(Self::ContainsCarbonFiber),
            32u16 => Ok(Self::ContainsCarbonNanoTubes),
            33u16 => Ok(Self::ContainsGlass),
            34u16 => Ok(Self::ContainsGlassFiber),
            35u16 => Ok(Self::ContainsKevlar),
            68u16 => Ok(Self::ContainsPtfe),
            36u16 => Ok(Self::ContainsStone),
            37u16 => Ok(Self::ContainsMagnetite),
            38u16 => Ok(Self::ContainsOrganicMaterial),
            39u16 => Ok(Self::ContainsCork),
            40u16 => Ok(Self::ContainsWax),
            41u16 => Ok(Self::ContainsWood),
            66u16 => Ok(Self::ContainsAlgae),
            42u16 => Ok(Self::ContainsBamboo),
            43u16 => Ok(Self::ContainsPine),
            44u16 => Ok(Self::ContainsCeramic),
            45u16 => Ok(Self::ContainsBoronCarbide),
            46u16 => Ok(Self::ContainsMetal),
            47u16 => Ok(Self::ContainsBronze),
            48u16 => Ok(Self::ContainsIron),
            49u16 => Ok(Self::ContainsSteel),
            50u16 => Ok(Self::ContainsSilver),
            51u16 => Ok(Self::ContainsCopper),
            52u16 => Ok(Self::ContainsAluminium),
            53u16 => Ok(Self::ContainsBrass),
            54u16 => Ok(Self::ContainsTungsten),
            55u16 => Ok(Self::ImitatesWood),
            56u16 => Ok(Self::ImitatesMetal),
            57u16 => Ok(Self::ImitatesMarble),
            58u16 => Ok(Self::ImitatesStone),
            59u16 => Ok(Self::Lithophane),
            60u16 => Ok(Self::Recycled),
            69u16 => Ok(Self::LimitedEdition),
            other => Err(Error::InvalidEnumDiscriminant(other)),
        }
    }
}
impl TryFrom<u8> for Tags {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > u16::MAX as u8 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<u32> for Tags {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > u16::MAX as u32 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<u64> for Tags {
    type Error = Error;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value > u16::MAX as u64 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<usize> for Tags {
    type Error = Error;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value > u16::MAX as usize {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i8> for Tags {
    type Error = Error;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i8 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i16> for Tags {
    type Error = Error;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i16 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i32> for Tags {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i32 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<i64> for Tags {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as i64 {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl TryFrom<isize> for Tags {
    type Error = Error;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value < 0 || value > u16::MAX as isize {
            return Err(Error::InvalidEnumDiscriminant(value as u16));
        }
        Self::try_from(value as u16)
    }
}
impl<'b, C> minicbor::Decode<'b, C> for Tags {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Tags, minicbor::decode::Error> {
        let pos = d.position();
        let n = d.i64()?;
        Tags::try_from(n).map_err(|_| minicbor::decode::Error::unknown_variant(n).at(pos))
    }
}
