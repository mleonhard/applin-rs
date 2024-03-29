use applin::widget::Real32;

#[test]
fn deserialize() {
    assert_eq!(
        serde_json::from_str::<Real32>("1").unwrap(),
        Real32::new(1.0)
    );
    assert_eq!(
        serde_json::from_str::<Real32>("1.0").unwrap(),
        Real32::new(1.0)
    );
    assert_eq!(
        serde_json::from_str::<Real32>("-1").unwrap(),
        Real32::new(-1.0)
    );
    assert_eq!(
        serde_json::from_str::<Real32>("999999999").unwrap(),
        Real32::new(999_999_999.0)
    );
    assert_eq!(
        serde_json::from_str::<Real32>("-999999999").unwrap(),
        Real32::new(-999_999_999.0)
    );
    assert_eq!(
        serde_json::from_str::<Real32>("0.9999999").unwrap(),
        Real32::new(0.999_999_9)
    );
    assert_eq!(
        serde_json::from_str::<Real32>("-0.9999999").unwrap(),
        Real32::new(-0.999_999_9)
    );
    assert_eq!(
        serde_json::from_str::<Real32>("0.99999999").unwrap(),
        Real32::new(1.0)
    );
    assert_eq!(
        serde_json::from_str::<Real32>("-0.99999999").unwrap(),
        Real32::new(-1.0)
    );
    serde_json::from_str::<Real32>("null").unwrap_err();
}
