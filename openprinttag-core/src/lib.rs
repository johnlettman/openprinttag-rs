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
 * */
///(no `std`, using `alloc`)
pub type EnumArray<T, const N: usize> = alloc::vec::Vec<T>;
#[cfg(all(not(feature = "std"), not(feature = "alloc")))]
/**An array of enum values.
 * */
///(no `std` and no `alloc`)
pub type EnumArray<T, const N: usize> = heapless::Vec<T, N>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected type")]
    UnexpectedType,
    #[error("invalid enum discriminant: {0}")]
    InvalidEnumDiscriminant(u32),
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
///Dynamic data, typically usage tracking.
pub struct Aux {
    /**Amount of material that was used up from the container

    `remaining_weight` = `instance_netto_full_weight` - `consumed_weight`*/
    pub consumed_weight: f32,
    ///Workgroup identifier, used for detecting first usage of the material.
    /// See the "write protection" section.
    pub workgroup: String,
    /**Determines semantics of the fields in the general purpose key range.

    MUST be filled if any of the general purpose keys is used.
    See "Vendor-specific fields"*/
    pub general_purpose_range_user: String,
    /**Timestamp when the resin was last stirred.

    Resins that have not been used for some time should be stirred before printing.*/
    pub last_stir_time: chrono::DateTime<chrono::Utc>,
}
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
#[repr(u32)]
///Contains material information that does not change during the package
/// instance lifetime.
pub enum MaterialClass {
    #[doc = "**Filament**\nFilament"]
    FFF = 0u32,
    #[doc = "**Resin**\nResin"]
    SLA = 1u32,
}
///Contains material information that does not change during the package
/// instance lifetime.
pub struct Main {
    /**Unique identifier of the package instance.

    If not specified, can be deduced from `brand_uuid` + NFC tag UID. See UUID section for more details.*/
    pub instance_uuid: uuid::Uuid,
    /**Universally unique identifier of the package (product)

    If not specified, can be deduced from `brand_uuid` + `gtin`. See UUID section for more details.*/
    pub package_uuid: uuid::Uuid,
    /**Universally unique identifier of the material

    If not specified, can be deduced from `brand_uuid` + `material_name`. See UUID section for more details.*/
    pub material_uuid: uuid::Uuid,
    /**Universally unique identifier of the brand

    If not specified, can be deduced from the `brand_name` string. See UUID section for more details.*/
    pub brand_uuid: uuid::Uuid,
    ///Global Trade Item Number
    pub gtin: f32,
    /**Brand-specific identifier of the package instance.

    Not much use cases at this moment, possibly just for URL deduction*/
    pub brand_specific_instance_id: String,
    /**Brand-specific identifier of the package (product ID)

    Not much use cases at this moment, possibly just for URL deduction*/
    pub brand_specific_package_id: String,
    /**Together with brand uniquely identifies each material

    Not much use cases at this moment, possibly just for URL deduction*/
    pub brand_specific_material_id: String,
    /**Brand-specific material display string/identifier.

    In the UI, brand_name + material_name should be displayed together, for example "Prusament PLA Galaxy Black"*/
    pub material_name: String,
    /**Abbreviation of the material name, for UI purposes (footers, dashboards, ...)

    If not present, the material inherits the abbreviation from the material type.*/
    pub material_abbreviation: String,
    ///Brand of the material
    pub brand_name: String,
    pub manufactured_date: chrono::DateTime<chrono::Utc>,
    ///Country the [MaterialPackageInstance](terminology) was produced in, encoded as a two-letter code according to [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    pub country_of_origin: String,
    pub expiration_date: chrono::DateTime<chrono::Utc>,
    /**Nominal/advertised weight of the full package of the material, excluding the container

    The actual netto weight of a spcific package instance can slightly differ and is specified by `actual_netto_full_weight`*/
    pub nominal_netto_full_weight: f32,
    /**Actual weight of the full package of the material of this specific package instance, excluding the weight of the container.

    Can slightly differ from `nominal_netto_full_weight`.
    If not present, it is assumed to match `nominal_netto_full_weight`.*/
    pub actual_netto_full_weight: f32,
    /**Nominal/advertised filament length of the full spool.

    The actual length of a specific package instance can slightly differ and is specified by `actual_full_length`*/
    pub nominal_full_length: f32,
    /**Actual filament length of the full spool.

    Can slightly differ from `netto_full_length`*/
    pub actual_full_length: f32,
    ///Weight of the empty container
    pub empty_container_weight: f32,
    /**Primary color of the material in the RGB(A) format, intended for UI purposes.

    The alpha channel can be left out, in which case the data should have 3 bytes instead of 4 and the color will be considered fully opaque.
    If a material doesn't have a single primary color (for example rainbow or coextruded filaments), this field can be null.*/
    pub primary_color: [u8; 4],
    /**One of secondary colors of the material.

    Data format is the same as for `primary_color`*/
    pub secondary_color_0: [u8; 4],
    ///See `secondary_color_0`
    pub secondary_color_1: [u8; 4],
    ///See `secondary_color_0`
    pub secondary_color_2: [u8; 4],
    ///See `secondary_color_0`
    pub secondary_color_3: [u8; 4],
    ///See `secondary_color_0`
    pub secondary_color_4: [u8; 4],
    /**Transmission Distance is a number representing material opacity. Value ranges from 0.1 (least transparent/most opaque) to 100 (most transparent/least opaque)

    See [Prusa TD values](https://help.prusa3d.com/article/hueforge-filament-transparency-values-and-hexcodes_762314) or [HueForge website](https://shop.thehueforge.com/blogs/news/what-is-hueforge).*/
    pub transmission_distance: f32,
    ///Density of the material
    pub density: f32,
    /**Diameter of the filament, in mm.

    If not present, 1.75 mm is assumed.*/
    pub filament_diameter: f32,
    /**Hardness of the material on the Shore A hardness scale (suitable for softer materials)

    Note: There is no 1:1 mapping between A and D scales, different materials can have different values on one scale even though they are the same on the other.*/
    pub shore_hardness_a: u32,
    /**Hardness of the material on the Shore D hardness scale (suitable for harder materials)

    Note: There is no 1:1 mapping between A and D scales, different materials can have different values on one scale even though they are the same on the other.*/
    pub shore_hardness_d: u32,
    /**Filaments can contain particles that would clog smaller nozzles.

    This field specifies minimum nozzle diameter recommended for printing this material.*/
    pub min_nozzle_diameter: f32,
    /**Minimum recommended nozzle tempeature for printing.

    Also used for loading the filament to the nozzle.*/
    pub min_print_temperature: u32,
    /**Maximum recommended nozzle tempeature for printing.

    Also used for loading the filament to the nozzle.*/
    pub max_print_temperature: u32,
    /**Recommended nozzle tempeature for preheating/loadcell bed leveling.

    Should be large enough for the material to get soft, but not low enough for it no to drip out of the nozzle.*/
    pub preheat_temperature: u32,
    ///Minimum recommended heatbed tempeature.
    pub min_bed_temperature: u32,
    ///Maximum recommended heatbed tempeature.
    pub max_bed_temperature: u32,
    ///Minimum recommended temperature of the chamber.
    pub min_chamber_temperature: u32,
    ///Maximum recommended temperature of the chamber.
    pub max_chamber_temperature: u32,
    ///Ideal chamber temperature for printing.
    pub chamber_temperature: u32,
    ///Width of the filament spool. Can be useful to know for spool holders,
    /// dryboxes and such.
    pub container_width: u32,
    ///Diameter of the spool. Can be useful to know for spool holders, dryboxes
    /// and such.
    pub container_outer_diameter: u32,
    /**Diameter of the inner cylinder the filament is spooled once.

    Equals to the minimum diameter of the filament winding.*/
    pub container_inner_diameter: u32,
    ///Diameter of the center hole of the spool.
    pub container_hole_diameter: u32,
    ///Viscosity of the material at 18 °C
    pub viscosity_18c: f32,
    ///Viscosity of the material at 25 °C
    pub viscosity_25c: f32,
    ///Viscosity of the material at 40 °C
    pub viscosity_40c: f32,
    ///Viscosity of the material at 60 °C
    pub viscosity_60c: f32,
    ///Maximum amount of material the container can hold.
    pub container_volumetric_capacity: f32,
    ///Wavelength of the light the material has been designed to be cured with
    pub cure_wavelength: u32,
}
#[repr(u32)]
///Contains material information that does not change during the package
/// instance lifetime.
pub enum Tags {
    #[doc = "**Filtration recommended**\nReleases a higher concentration of unsafe particles/fumes during printing so a HEPA and carbon filter is strongly recommended."]
    FiltrationRecommended = 0u32,
    #[doc = "**Biocompatible**\nCertified biocompatibility (does not cause harmful effects when in contact with the body)"]
    Biocompatible = 1u32,
    #[doc = "**Home compostable**\nDecomposes into natural elements in a home compost system at ambient temperatures."]
    HomeCompostable = 61u32,
    #[doc = "**Industrially compostable**\nDecomposes into natural elements under specific temperature and microbial conditions in commercial composting facilities."]
    IndustriallyCompostable = 62u32,
    #[doc = "**Bio-based**\nPredominantly made from renewable biological resources, like plants."]
    BioBased = 63u32,
    #[doc = "**Antibacterial**\nHas antibacterial properties"]
    Antibacterial = 2u32,
    #[doc = "**Air filtering**\nHas air filtering properties (absorbs/filters harmful compounds/particles from the air)"]
    AirFiltering = 3u32,
    #[doc = "**Abrasive**\nThe material is abrasive and requires an abrasive-resistant nozzle."]
    Abrasive = 4u32,
    #[doc = "**Foaming**\nThe material increases its volume during extrusion"]
    Foaming = 5u32,
    #[doc = "**Castable**\nThe material is suitable to be used as a sacrificial pattern for investement casting.\n\nIt can be cleanly removed from the mold (typically burned out or melted away), leaving minimal residue (for example ashes).\nThis does NOT mean that the material is used for the mold or the final cast itself, only the investment pattern."]
    Castable = 67u32,
    #[doc = "**Self-extinguishing**\nThe material is self-extinguishing. This does not mean that the material is not flammable, just that burning it implies more energy than it produces.\n\nMeets at least UL 94 HB."]
    SelfExtinguishing = 6u32,
    #[doc = "**Paramagnetic**\nThe material has paramagnetic properties, meaning that it is (weakly) attracted to magnets."]
    Paramagnetic = 7u32,
    #[doc = "**Radiation shielding**\nHas radiation shielding properties"]
    RadiationShielding = 8u32,
    #[doc = "**High temperature**\nThe material softens at higher temperatures than what is common for the material type, while keeping similar printing temperatures.\n\nCan be used for HTPLA filament while keeping the PLA material type.\nThis does NOT indicate increase resistance to flame/burning.\nNote: If the material type would be 'HTPLA', adding this tag would mean 'high-temperature variant of a high-temperature PLA'"]
    HighTemperature = 9u32,
    #[doc = "**ESD safe**\nThe material is static dissipative - prevents electrostatic charge buildup by allowing gradual dissipation of the charge.\n\nUseful for protecting sensitive electronic components.\nSheet resistance R >= 1e5 Ω/□ && R < 1e12 Ω/□ or volumetric resistivity ρ >= 1e4 Ω⋅cm && ρ < 1e11 Ω⋅cm.\nThe tag does NOT cover the \"anti-static\" materials (that have higher resistances)."]
    ESDSafe = 10u32,
    #[doc = "**Conductive**\nThe material can conduct electricity.\n\nThis does NOT mean that it the material is a good conductor, such as metals. Common \"conductive\" material have resistances in the range of kiloohms on 10 cm of filament.\nSheet resistance R < 1e5 Ω/□ or volumetric resistivity ρ < 1e4 Ω⋅cm."]
    Conductive = 11u32,
    #[doc = "**EMI shielding**\nThe material can be effectively used for shielding against electromagnetic interference.\n\nSheet resistance R < 1 Ω/□ or volumetric resistivity ρ < 1e-2 Ω⋅cm.\n\n**Implies:** [`Conductive`][ConfigNfcv::Conductive]"]
    EMIShielding = 70u32,
    #[doc = "**Blend**\nThe material is a blend of multiple polymers or a base polymer with significant additives that alter its properties and may require a specific print profile."]
    Blend = 12u32,
    #[doc = "**Water soluble**\nCan be dissolved in water"]
    WaterSoluble = 13u32,
    #[doc = "**IPA soluble**\nCan be dissolved in IPA (isopropylalcohol)"]
    IpaSoluble = 14u32,
    #[doc = "**Limonene soluble**\nCan be dissolved in limonene"]
    LimoneneSoluble = 15u32,
    #[doc = "**Low outgassing**\nReleases only minimal gas (and vapor) when placed in a vacuum."]
    LowOutgassing = 64u32,
    #[doc = "**Matte**\nProduces matte, non-shiny surface (very low specular reflection coefficient)"]
    Matte = 16u32,
    #[doc = "**Silk**\nProduces smooth, shiny/glossy surface (higher specular reflection coefficient)"]
    Silk = 17u32,
    #[doc = "**Translucent**\nNot fully opaque – [HueForge TD](https://shop.thehueforge.com/blogs/news/what-is-hueforge) >= X (exact X will be determined later)\n\nThe material with this tag can possibly disperse light, meaning that while the light goes through it, the image is \"blurred\" and one does not see clearly what's on the other side. See the `transparent` tag."]
    Translucent = 19u32,
    #[doc = "**Transparent**\nNot fully opaque, does not disperse light.\n\nUnder correct printing conditions, can be printed with a see-through glass-like transparency.\n\n**Implies:** [`Translucent`][ConfigNfcv::Translucent]"]
    Transparent = 20u32,
    #[doc = "**Without pigments**\nThe material is of its \"natural\" color, no pigments were added\n\n**Hints:** [`Translucent`][ConfigNfcv::Translucent]"]
    WithoutPigments = 65u32,
    #[doc = "**Iridescent**\nSame as mystic\n\nChanges color based on the viewing angle\nSee https://en.wikipedia.org/wiki/Iridescence"]
    Iridescent = 21u32,
    #[doc = "**Pearlescent**\nSpecial case of iridescent where the reflected light is mostly white\n\nSee https://en.wikipedia.org/wiki/Iridescence#Pearlescence\n\n**Implies:** [`Iridescent`][ConfigNfcv::Iridescent]"]
    Pearlescent = 22u32,
    #[doc = "**Glitter**\nContains coarse glitter particles, causing a shimmering effect.\n\nSimilar to iridescent/pearlescent, but the individual particles causing the effect are larger, visible with the naked eye"]
    Glitter = 23u32,
    #[doc = "**Glow in the dark**\nGlows in the dark (phosphorescent).\n\nThe glow color doesn't necessarily match the base material color (`illuminescent_color_change`)\nThe different glow color can be specified as secondary color of the material."]
    GlowInTheDark = 24u32,
    #[doc = "**Neon**\nNeon color/glows under UV light (fluorescent)\n\nThe glow color doesn't necessarily match the base material color (`illuminescent_color_change`)\nThe different glow color can be specified as secondary color of the material."]
    Neon = 25u32,
    #[doc = "**Illuminescent color change**\nThe glow color (caused by illuminiscence) is different to the material base color.\n\nFor example the material is blue, but glows green in the dark or under the UV light.\nThe glow color can be specified as a secondary color of the material."]
    IlluminescentColorChange = 26u32,
    #[doc = "**Temperature color change**\nChanges color based on the temperature."]
    TemperatureColorChange = 27u32,
    #[doc = "**Gradual color change**\nTransitions between colors as the filament is extruded.\n\nDoes not necessary mean that the filament must go through the rainbow colors, gradual color change between two colors is enough to qualify"]
    GradualColorChange = 28u32,
    #[doc = "**Coextruded**\nCo-extruded from multiple colors. The colors are all present at any cross-section of the filament.\n\nDo not confuse with `gradual_color_change`\nDoes not have a primary color, number of colors can be derived from the defined secondary colors."]
    Coextruded = 29u32,
    #[doc = "**Contains carbon**\nContains carbon"]
    ContainsCarbon = 30u32,
    #[doc = "**Contains carbon fiber**\nContains carbon fibers\n\n**Implies:** [`ContainsCarbon`][ConfigNfcv::ContainsCarbon]"]
    ContainsCarbonFiber = 31u32,
    #[doc = "**Contains carbon nano tubes**\nContains carbon nano tubes\n\nNote: The name 'nano tubes' describes the diameter, but the tubes are typically several micrometers long, so this tag actually implies 'particles_micro'\n\n**Implies:** [`ContainsCarbon`][ConfigNfcv::ContainsCarbon]"]
    ContainsCarbonNanoTubes = 32u32,
    #[doc = "**Contains glass**\nContains glass"]
    ContainsGlass = 33u32,
    #[doc = "**Contains glass fiber**\nContains glass fibers\n\n**Implies:** [`ContainsGlass`][ConfigNfcv::ContainsGlass]"]
    ContainsGlassFiber = 34u32,
    #[doc = "**Contains Kevlar**\nContains kevlar (aramid)"]
    ContainsKevlar = 35u32,
    #[doc = "**Contains PTFE**\n"]
    ContainsPTFE = 68u32,
    #[doc = "**Contains stone**\n\n\n**Hints:** [`Abrasive`][ConfigNfcv::Abrasive]"]
    ContainsStone = 36u32,
    #[doc = "**Contains magnetite**\n\n\n**Hints:** [`Abrasive`][ConfigNfcv::Abrasive]"]
    ContainsMagnetite = 37u32,
    #[doc = "**Contains organic material**\n"]
    ContainsOrganicMaterial = 38u32,
    #[doc = "**Contains cork**\n\n\n**Implies:** [`ContainsOrganicMaterial`][ConfigNfcv::ContainsOrganicMaterial]"]
    ContainsCork = 39u32,
    #[doc = "**Contains wax**\n\n\n**Implies:** [`ContainsOrganicMaterial`][ConfigNfcv::ContainsOrganicMaterial]"]
    ContainsWax = 40u32,
    #[doc = "**Contains wood**\n\n\n**Implies:** [`ContainsOrganicMaterial`][ConfigNfcv::ContainsOrganicMaterial]"]
    ContainsWood = 41u32,
    #[doc = "**Contains algae**\n\n\n**Implies:** [`ContainsOrganicMaterial`][ConfigNfcv::ContainsOrganicMaterial]"]
    ContainsAlgae = 66u32,
    #[doc = "**Contains bamboo**\n\n\n**Implies:** [`ContainsWood`][ConfigNfcv::ContainsWood]"]
    ContainsBamboo = 42u32,
    #[doc = "**Contains pine**\n\n\n**Implies:** [`ContainsWood`][ConfigNfcv::ContainsWood]"]
    ContainsPine = 43u32,
    #[doc = "**Contains ceramic**\n\n\n**Hints:** [`Abrasive`][ConfigNfcv::Abrasive]"]
    ContainsCeramic = 44u32,
    #[doc = "**Contains boron carbide**\n\n\n**Hints:** [`RadiationShielding`][ConfigNfcv::RadiationShielding]\n**Implies:** [`ContainsCeramic`][ConfigNfcv::ContainsCeramic]"]
    ContainsBoronCarbide = 45u32,
    #[doc = "**Contains metal**\nContains metal. Specific type of metal contained can be expressed by an other tag.\n\n**Hints:** [`Abrasive`][ConfigNfcv::Abrasive]"]
    ContainsMetal = 46u32,
    #[doc = "**Contains bronze**\n\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsBronze = 47u32,
    #[doc = "**Contains iron**\n\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsIron = 48u32,
    #[doc = "**Contains steel**\n\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsSteel = 49u32,
    #[doc = "**Contains silver**\n\n\n**Hints:** [`Antibacterial`][ConfigNfcv::Antibacterial]\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsSilver = 50u32,
    #[doc = "**Contains copper**\n\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsCopper = 51u32,
    #[doc = "**Contains aluminium**\n\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsAluminium = 52u32,
    #[doc = "**Contains brass**\n\n\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsBrass = 53u32,
    #[doc = "**Contains tungsten**\nContains Tungsten (useful for radiation shielding).\n\n**Hints:** [`RadiationShielding`][ConfigNfcv::RadiationShielding]\n**Implies:** [`ContainsMetal`][ConfigNfcv::ContainsMetal]"]
    ContainsTungsten = 54u32,
    #[doc = "**Imitates wood**\nImitates wood"]
    ImitatesWood = 55u32,
    #[doc = "**Imitates metal**\nImitates metal"]
    ImitatesMetal = 56u32,
    #[doc = "**Imitates marble**\nImitates marble"]
    ImitatesMarble = 57u32,
    #[doc = "**Imitates stone**\nImitates stone"]
    ImitatesStone = 58u32,
    #[doc = "**Lithophane**\nSpecifically designed for lithophaning"]
    Lithophane = 59u32,
    #[doc = "**Recycled**\nPart of the material is recycled"]
    Recycled = 60u32,
    #[doc = "**Limited edition**\nThe material is a limited edition run"]
    LimitedEdition = 69u32,
}
#[repr(u32)]
///Contains material information that does not change during the package
/// instance lifetime.
pub enum MaterialType {
    #[doc = "**Polylactic Acid**\nEasy-to-print, biodegradable material. Ideal for beginners, prototypes, and models."]
    PLA = 0u32,
    #[doc = "**Polyethylene Terephthalate Glycol**\nDurable, strong, and temperature-resistant. Great for mechanical parts and functional prints."]
    PETG = 1u32,
    #[doc = "**Thermoplastic Polyurethane**\nA flexible, rubber-like material. Used for phone cases, vibration dampeners, and other soft parts."]
    TPU = 2u32,
    #[doc = "**Acrylonitrile Butadiene Styrene**\nStrong, durable, and heat-resistant plastic. Used for functional parts like car interiors and LEGOs. Requires a heated bed and enclosure."]
    ABS = 3u32,
    #[doc = "**Acrylonitrile Styrene Acrylate**\nSimilar to ABS but with high UV and weather resistance, making it perfect for outdoor applications."]
    ASA = 4u32,
    #[doc = "**Polycarbonate**\nExtremely strong, impact-resistant, and heat-resistant. Used for demanding engineering applications."]
    PC = 5u32,
    #[doc = "**Polycyclohexylenedimethylene Terephthalate Glycol**\nA tougher alternative to PETG with higher impact and chemical resistance."]
    PCTG = 6u32,
    #[doc = "**Polypropylene**\nLightweight, chemically resistant, and flexible. Used for creating living hinges and durable containers."]
    PP = 7u32,
    #[doc = "**Polyamide 6**\nA type of Nylon that is tough and wear-resistant but absorbs more moisture than other nylons."]
    PA6 = 8u32,
    #[doc = "**Polyamide 11**\nA flexible, bio-based Nylon with low moisture absorption and good chemical resistance."]
    PA11 = 9u32,
    #[doc = "**Polyamide 12**\nThe most common Nylon for 3D printing. Strong, tough, with low moisture absorption. Great for functional parts."]
    PA12 = 10u32,
    #[doc = "**Polyamide 66**\nA stiffer and more heat-resistant Nylon compared to PA6, used for durable mechanical parts."]
    PA66 = 11u32,
    #[doc = "**Copolyester**\nA family of strong and dimensionally stable materials (including PETG) known for chemical resistance."]
    CPE = 12u32,
    #[doc = "**Thermoplastic Elastomer**\nA general class of soft, rubbery materials. Softer and more flexible than TPU."]
    TPE = 13u32,
    #[doc = "**High Impact Polystyrene**\nA lightweight material often used as a dissolvable support material for ABS prints (dissolves in Limonene)."]
    HIPS = 14u32,
    #[doc = "**Polyhydroxyalkanoate**\nA biodegradable material similar to PLA but with better toughness and flexibility."]
    PHA = 15u32,
    #[doc = "**Polyethylene Terephthalate**\nThe same plastic used in water bottles. Strong and food-safe, but less common for printing than PETG."]
    PET = 16u32,
    #[doc = "**Polyetherimide**\nA high-performance material (also known as Ultem) with excellent thermal and mechanical properties."]
    PEI = 17u32,
    #[doc = "**Polybutylene Terephthalate**\nAn engineering polymer with good heat resistance and electrical insulation properties."]
    PBT = 18u32,
    #[doc = "**Polyvinyl Butyral**\nEasy to print and can be chemically smoothed with isopropyl alcohol for a glossy finish."]
    PVB = 19u32,
    #[doc = "**Polyvinyl Alcohol**\nA water-soluble filament used exclusively as a support material for complex prints."]
    PVA = 20u32,
    #[doc = "**Polyetherketoneketone**\nAn ultra-high-performance polymer with exceptional heat, chemical, and mechanical properties for industrial use."]
    PEKK = 21u32,
    #[doc = "**Polyether Ether Ketone**\nAn ultra-high-performance polymer with exceptional mechanical, thermal, and chemical resistance. Used in demanding aerospace, medical, and industrial applications."]
    PEEK = 22u32,
    #[doc = "**Butenediol Vinyl Alcohol Copolymer**\nA water-soluble support material that often dissolves faster and is easier to print than PVA."]
    BVOH = 23u32,
    #[doc = "**Thermoplastic Copolyester**\nA flexible, TPE-like material with good thermal and chemical resistance."]
    TPC = 24u32,
    #[doc = "**Polyphenylene Sulfide**\nA high-performance polymer known for its thermal stability and chemical resistance, often used in automotive and electronics."]
    PPS = 25u32,
    #[doc = "**Polyphenylsulfone**\nA high-performance material with excellent heat and chemical resistance, often used in medical applications."]
    PPSU = 26u32,
    #[doc = "**Polyvinyl Chloride**\nStrong and durable but rarely used in 3D printing due to the release of toxic fumes."]
    PVC = 27u32,
    #[doc = "**Polyether Block Amide**\nA flexible and lightweight TPE known for its excellent energy return, used in sports equipment."]
    PEBA = 28u32,
    #[doc = "**Polyvinylidene Fluoride**\nHigh-performance polymer with excellent resistance to chemicals and UV light."]
    PVDF = 29u32,
    #[doc = "**Polyphthalamide**\nA high-performance Nylon with superior strength, stiffness, and heat resistance compared to standard Nylons."]
    PPA = 30u32,
    #[doc = "**Polycaprolactone**\nA biodegradable polyester with a very low melting point (~60 °C), allowing it to be reshaped by hand in hot water."]
    PCL = 31u32,
    #[doc = "**Polyethersulfone**\nA high-temperature, amorphous polymer with good chemical and hydrolytic stability."]
    PES = 32u32,
    #[doc = "**Polymethyl Methacrylate**\nA rigid, transparent material also known as acrylic. Offers good optical clarity."]
    PMMA = 33u32,
    #[doc = "**Polyoxymethylene**\nA low-friction, rigid material also known as Delrin. Excellent for gears, bearings, and moving parts."]
    POM = 34u32,
    #[doc = "**Polyphenylene Ether**\nAn engineering thermoplastic with good temperature resistance and dimensional stability, often used in blends."]
    PPE = 35u32,
    #[doc = "**Polystyrene**\nA lightweight and brittle material. Not commonly used in its pure form for 3D printing."]
    PS = 36u32,
    #[doc = "**Polysulfone**\nA high-temperature material with good thermal stability and chemical resistance."]
    PSU = 37u32,
    #[doc = "**Thermoplastic Polyimide**\nAn ultra-high-performance polymer with one of the highest glass transition temperatures and excellent thermal stability."]
    TPI = 38u32,
    #[doc = "**Styrene-Butadiene-Styrene**\nA flexible, rubber-like material (a type of TPE) known for good durability. It is relatively easy to print for a flexible filament."]
    SBS = 39u32,
}
#[repr(u32)]
///Contains material information that does not change during the package
/// instance lifetime.
pub enum WriteProtection {
    ///The tag is not write protected.
    No = 0u32,
    ///The tag is irreversibly protected against writing.
    Irreversible = 1u32,
    ///The tag is write-protected using the `PROTECT PAGE` command
    /// (SLIX2-specific) and is unlockable with a password that is located
    /// somewhere on the container.
    ProtectPageUnlockable = 2u32,
}
