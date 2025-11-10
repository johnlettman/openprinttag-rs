use openprinttag_codegen::name::to_camel;

#[test]
fn test_to_enum_variant() {
    struct TestCase {
        pub name: &'static str,
        pub want: &'static str,
    }

    let cases = vec![
        TestCase { name: "abrasive", want: "Abrasive" },
        TestCase { name: "castable", want: "Castable" },
        TestCase { name: "self_extinguishing", want: "SelfExtinguishing" },
        TestCase { name: "radiation_shielding", want: "RadiationShielding" },
        TestCase { name: "contains_organic_material", want: "ContainsOrganicMaterial" },
        TestCase { name: "contains_glass_fiber", want: "ContainsGlassFiber" },
        // special caps cases
        TestCase { name: "esd_safe", want: "ESDSafe" },
        TestCase { name: "contains_ptfe", want: "ContainsPTFE" },
    ];

    for case in cases {
        let got = to_camel(case.name);
        assert_eq!(got, case.want);
    }
}
