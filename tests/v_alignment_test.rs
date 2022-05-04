use maggie::widget::VAlignment;
use serde_json::Value;

#[test]
fn valignment_to_value() {
    assert_eq!(VAlignment::Top.to_value(), Value::String("top".to_string()));
}

#[test]
fn value_from_valignment() {
    let value: Value = VAlignment::Top.into();
    assert_eq!(value, Value::String("top".to_string()));
}

#[test]
fn valignment_default() {
    assert_eq!(VAlignment::default(), VAlignment::Top);
}

#[test]
fn valignment_serialize() {
    assert_eq!(serde_json::to_string(&VAlignment::Top).unwrap(), "\"top\"");
    assert_eq!(
        serde_json::to_string(&VAlignment::Center).unwrap(),
        "\"center\""
    );
    assert_eq!(
        serde_json::to_string(&VAlignment::Bottom).unwrap(),
        "\"bottom\""
    );
}

#[test]
fn valignment_deserialize() {
    assert_eq!(
        serde_json::from_str::<VAlignment>("\"top\"").unwrap(),
        VAlignment::Top
    );
    assert_eq!(
        serde_json::from_str::<VAlignment>("\"center\"").unwrap(),
        VAlignment::Center
    );
    assert_eq!(
        serde_json::from_str::<VAlignment>("\"bottom\"").unwrap(),
        VAlignment::Bottom
    );
}
