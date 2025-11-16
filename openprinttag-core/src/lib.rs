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
    pub meta: Meta,
    pub main: Main,
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
#[derive(Debug, Clone, minicbor_derive::Encode, minicbor_derive::Decode)]
///Dynamic data, typically usage tracking.
pub struct Aux {
    #[n(0u32)]
    /**Amount of material that was used up from the container.

    `remaining_weight` = `instance_netto_full_weight` - `consumed_weight`*/
    pub consumed_weight: f32,
    #[n(1u32)]
    /**Workgroup identifier, used for detecting first usage of the material.

    See the _write protection_ section.*/
    pub workgroup: String,
    #[n(2u32)]
    /**Determines semantics of the fields in the general purpose key range.

    MUST be filled if any of the general purpose keys is used.
    See _Vendor-specific fields_.*/
    pub general_purpose_range_user: String,
    #[n(3u32)]
    /**Timestamp when the resin was last stirred.

    Resins that have not been used for some time should be stirred before printing.*/
    pub last_stir_time: chrono::DateTime<chrono::Utc>,
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, minicbor_derive::Encode, minicbor_derive::Decode)]
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
    ESDSafe = 10u16,
    #[doc = "**Conductive**\nThe material can conduct electricity.\n\nThis does NOT mean that it the material is a good conductor, such as metals.\nCommon \"conductive\" material have resistances in the range of kiloohms on 10 cm of filament.\nSheet resistance R < 1e5 Ω/□ or volumetric resistivity ρ < 1e4 Ω⋅cm."]
    Conductive = 11u16,
    #[doc = "**EMI shielding**\nThe material can be effectively used for shielding against electromagnetic interference.\n\nSheet resistance R < 1 Ω/□ or volumetric resistivity ρ < 1e-2 Ω⋅cm.\n\n**Implies:** [`Conductive`][ConfigNfcv::Conductive]"]
    EMIShielding = 70u16,
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
    ContainsPTFE = 68u16,
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
            10u16 => Ok(Self::ESDSafe),
            11u16 => Ok(Self::Conductive),
            70u16 => Ok(Self::EMIShielding),
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
            68u16 => Ok(Self::ContainsPTFE),
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
impl<'b, C> minicbor::Decode<'b, C> for MaterialType {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Tags, minicbor::decode::Error> {
        let pos = d.position();
        let n = d.i64()?;
        Tags::try_from(n).map_err(|_| minicbor::decode::Error::unknown_variant(n).at(pos))
    }
}
#[derive(Debug, Clone, minicbor_derive::Encode, minicbor_derive::Decode)]
///Contains material information that does not change during the package instance lifetime.
pub struct Main {
    #[n(0u32)]
    /**Unique identifier of the package instance.

    If not specified, can be deduced from `brand_uuid` + NFC tag UID.
    See _UUID_ section for more details.*/
    pub instance_uuid: uuid::Uuid,
    #[n(1u32)]
    /**Universally unique identifier of the package (product)

    If not specified, can be deduced from `brand_uuid` + `gtin`.
    See _UUID_ section for more details.*/
    pub package_uuid: uuid::Uuid,
    #[n(2u32)]
    /**Universally unique identifier of the material.

    If not specified, can be deduced from `brand_uuid` + `material_name`.
    See _UUID_ section for more details.*/
    pub material_uuid: uuid::Uuid,
    #[n(3u32)]
    /**Universally unique identifier of the brand

    If not specified, can be deduced from the `brand_name` string.
    See _UUID_ section for more details.*/
    pub brand_uuid: uuid::Uuid,
    #[n(4u32)]
    ///Global Trade Item Number.
    pub gtin: f32,
    #[n(5u32)]
    /**Brand-specific identifier of the package instance.

    Not much use cases at this moment, possibly just for URL deduction*/
    pub brand_specific_instance_id: String,
    #[n(6u32)]
    /**Brand-specific identifier of the package (product ID).

    Not much use cases at this moment, possibly just for URL deduction*/
    pub brand_specific_package_id: String,
    #[n(7u32)]
    /**Together with brand uniquely identifies each material.

    Not much use cases at this moment, possibly just for URL deduction.*/
    pub brand_specific_material_id: String,
    #[n(10u32)]
    /**Brand-specific material display string/identifier.

    In the UI, brand_name + material_name should be displayed together, for example "Prusament PLA Galaxy Black".*/
    pub material_name: String,
    #[n(52u32)]
    /**Abbreviation of the material name, for UI purposes (footers, dashboards, ...).

    If not present, the material inherits the abbreviation from the material type.*/
    pub material_abbreviation: String,
    #[n(11u32)]
    ///Brand of the material.
    pub brand_name: String,
    #[n(14u32)]
    pub manufactured_date: chrono::DateTime<chrono::Utc>,
    #[n(55u32)]
    ///Country the [MaterialPackageInstance](terminology) was produced in, encoded as a two-letter code according to [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2).
    pub country_of_origin: String,
    #[n(15u32)]
    pub expiration_date: chrono::DateTime<chrono::Utc>,
    #[n(16u32)]
    /**Nominal/advertised weight of the full package of the material, excluding the container.

    The actual netto weight of a specific package instance can slightly differ and is specified by `actual_netto_full_weight`.*/
    pub nominal_netto_full_weight: f32,
    #[n(17u32)]
    /**Actual weight of the full package of the material of this specific package instance, excluding the weight of the container.

    Can slightly differ from `nominal_netto_full_weight`.
    If not present, it is assumed to match `nominal_netto_full_weight`.*/
    pub actual_netto_full_weight: f32,
    #[n(53u32)]
    /**Nominal/advertised filament length of the full spool.

    The actual length of a specific package instance can slightly differ and is specified by `actual_full_length`*/
    pub nominal_full_length: f32,
    #[n(54u32)]
    /**Actual filament length of the full spool.

    Can slightly differ from `netto_full_length`.*/
    pub actual_full_length: f32,
    #[n(18u32)]
    ///Weight of the empty container.
    pub empty_container_weight: f32,
    #[n(19u32)]
    /**Primary color of the material in the RGB(A) format, intended for UI purposes.

    The alpha channel can be left out, in which case the data should have 3 bytes instead of 4 and the color will be considered fully opaque.
    If a material doesn't have a single primary color (for example rainbow or coextruded filaments), this field can be null.*/
    pub primary_color: [u8; 4],
    #[n(20u32)]
    /**One of secondary colors of the material.

    Data format is the same as for `primary_color`.*/
    pub secondary_color_0: [u8; 4],
    #[n(21u32)]
    ///See `secondary_color_0`.
    pub secondary_color_1: [u8; 4],
    #[n(22u32)]
    ///See `secondary_color_0`.
    pub secondary_color_2: [u8; 4],
    #[n(23u32)]
    ///See `secondary_color_0`.
    pub secondary_color_3: [u8; 4],
    #[n(24u32)]
    ///See `secondary_color_0`.
    pub secondary_color_4: [u8; 4],
    #[n(27u32)]
    /**Transmission Distance is a number representing material opacity.

    Value ranges from 0.1 (least transparent/most opaque) to 100 (most transparent/least opaque).
    See [Prusa TD values](https://help.prusa3d.com/article/hueforge-filament-transparency-values-and-hexcodes_762314) or [HueForge website](https://shop.thehueforge.com/blogs/news/what-is-hueforge).*/
    pub transmission_distance: f32,
    #[n(28u32)]
    ///Properties of the material. Can have multiple tags at once.
    pub tags: EnumArray<Tags, 71>,
    #[n(29u32)]
    ///Density of the material.
    pub density: f32,
    #[n(30u32)]
    /**Diameter of the filament, in mm.

    If not present, 1.75 mm is assumed.*/
    pub filament_diameter: f32,
    #[n(31u32)]
    /**Hardness of the material on the Shore A hardness scale (suitable for softer materials).

    **Note:** There is no 1:1 mapping between A and D scales, different materials can have different values on one scale even though they are the same on the other.*/
    pub shore_hardness_a: u32,
    #[n(32u32)]
    /**Hardness of the material on the Shore D hardness scale (suitable for harder materials).

    **Note:** There is no 1:1 mapping between A and D scales, different materials can have different values on one scale even though they are the same on the other.*/
    pub shore_hardness_d: u32,
    #[n(33u32)]
    /**Filaments can contain particles that would clog smaller nozzles.

    This field specifies minimum nozzle diameter recommended for printing this material.*/
    pub min_nozzle_diameter: f32,
    #[n(34u32)]
    /**Minimum recommended nozzle temperature for printing.

    Also used for loading the filament to the nozzle.*/
    pub min_print_temperature: u32,
    #[n(35u32)]
    /**Maximum recommended nozzle temperature for printing.

    Also used for loading the filament to the nozzle.*/
    pub max_print_temperature: u32,
    #[n(36u32)]
    /**Recommended nozzle temperature for preheating/load cell bed leveling.

    Should be large enough for the material to get soft, but not low enough for it no to drip out of the nozzle.*/
    pub preheat_temperature: u32,
    #[n(37u32)]
    ///Minimum recommended heatbed temperature.
    pub min_bed_temperature: u32,
    #[n(38u32)]
    ///Maximum recommended heatbed temperature.
    pub max_bed_temperature: u32,
    #[n(39u32)]
    ///Minimum recommended temperature of the chamber.
    pub min_chamber_temperature: u32,
    #[n(40u32)]
    ///Maximum recommended temperature of the chamber.
    pub max_chamber_temperature: u32,
    #[n(41u32)]
    ///Ideal chamber temperature for printing.
    pub chamber_temperature: u32,
    #[n(42u32)]
    ///Width of the filament spool. Can be useful to know for spool holders, dry boxes and such.
    pub container_width: u32,
    #[n(43u32)]
    ///Diameter of the spool. Can be useful to know for spool holders, dry boxes and such.
    pub container_outer_diameter: u32,
    #[n(44u32)]
    /**Diameter of the inner cylinder the filament is spooled once.

    Equals to the minimum diameter of the filament winding.*/
    pub container_inner_diameter: u32,
    #[n(45u32)]
    ///Diameter of the center hole of the spool.
    pub container_hole_diameter: u32,
    #[n(46u32)]
    ///Viscosity of the material at 18 °C.
    pub viscosity_18c: f32,
    #[n(47u32)]
    ///Viscosity of the material at 25 °C.
    pub viscosity_25c: f32,
    #[n(48u32)]
    ///Viscosity of the material at 40 °C.
    pub viscosity_40c: f32,
    #[n(49u32)]
    ///Viscosity of the material at 60 °C.
    pub viscosity_60c: f32,
    #[n(50u32)]
    ///Maximum amount of material the container can hold.
    pub container_volumetric_capacity: f32,
    #[n(51u32)]
    ///Wavelength of the light the material has been designed to be cured with.
    pub cure_wavelength: u32,
}
#[derive(Debug, Clone, minicbor_derive::Encode, minicbor_derive::Decode)]
///Offsets and sizes within the `NDEF` payload.
pub struct Meta {
    #[n(0u32)]
    /**Offset of the main region, relative to the NDEF payload start.

    If not specified, the main region immediately follows the meta section.*/
    pub main_region_offset: u32,
    #[n(1u32)]
    /**Allocation size of the main region.

    If not specified, the region spans till the next region or payload end.*/
    pub main_region_size: u32,
    #[n(2u32)]
    /**Offset of the auxiliary region, relative to the NDEF payload start.

    Omitting this field means that the auxiliary region is not present.*/
    pub aux_region_offset: u32,
    #[n(3u32)]
    /**Allocation size of the auxiliary region.

    If not specified, the region (if present) spans till the next region or payload end.*/
    pub aux_region_size: u32,
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, minicbor_derive::Encode, minicbor_derive::Decode)]
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
impl<'b, C> minicbor::Decode<'b, C> for MaterialType {
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, minicbor_derive::Encode, minicbor_derive::Decode)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, minicbor_derive::Encode, minicbor_derive::Decode)]
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
impl<'b, C> minicbor::Decode<'b, C> for MaterialType {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<MaterialClass, minicbor::decode::Error> {
        let pos = d.position();
        let n = d.i64()?;
        MaterialClass::try_from(n).map_err(|_| minicbor::decode::Error::unknown_variant(n).at(pos))
    }
}
