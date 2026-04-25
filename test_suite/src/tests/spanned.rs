use xavier::{from_obj, from_xml, PError, Spanned, XmlDeserializable, XmlSerializable};

#[test]
fn read_spanned() -> Result<(), PError> {
    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name = "test_child")]
    struct TestValueChild {
        pub value: String,
    }

    #[derive(XmlDeserializable, XmlSerializable, Debug, PartialEq)]
    #[xml(name = "test_object", case = "Camel")]
    struct TestValueObject {
        pub id: u32,
        pub name: String,
        #[xml(tree)]
        pub child: TestValueChild,
        pub value_a: Spanned<String>,
        pub value_b: String,
    }

    let test_data = TestValueObject {
        id: 1,
        name: "Test Value Object".to_string(),
        child: TestValueChild {
            value: "Child Value".to_string(),
        },
        value_a: Spanned {
            span: 0..0,
            inner: "Value A".to_string(),
        },
        value_b: "Value B".to_string(),
    };

    let xml = from_obj(&test_data);

    assert!(xml.contains("<testObject>"));
    assert!(xml.contains("</testObject>"));
    assert!(xml.contains("<id>1</id>"));
    assert!(xml.contains("<name>Test Value Object</name>"));
    assert!(xml.contains("<child>"));
    assert!(xml.contains("</child>"));

    let parsed: Spanned<TestValueObject> = from_xml(&xml)?;
    assert_eq!(test_data, parsed.inner);

    Ok(())
}
