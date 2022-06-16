use applin::widget::HAlignment;
use serde_json::Value;

#[test]
fn halignment_to_value() {
    assert_eq!(
        HAlignment::Start.to_value(),
        Value::String("start".to_string())
    );
}

#[test]
fn value_from_halignment() {
    let value: Value = HAlignment::Start.into();
    assert_eq!(value, Value::String("start".to_string()));
}

#[test]
fn halignment_default() {
    assert_eq!(HAlignment::default(), HAlignment::Start);
}

#[test]
fn halignment_serialize() {
    assert_eq!(
        serde_json::to_string(&HAlignment::Start).unwrap(),
        "\"start\""
    );
    assert_eq!(
        serde_json::to_string(&HAlignment::Center).unwrap(),
        "\"center\""
    );
    assert_eq!(serde_json::to_string(&HAlignment::End).unwrap(), "\"end\"");
}

#[test]
fn halignment_deserialize() {
    assert_eq!(
        serde_json::from_str::<HAlignment>("\"start\"").unwrap(),
        HAlignment::Start
    );
    assert_eq!(
        serde_json::from_str::<HAlignment>("\"center\"").unwrap(),
        HAlignment::Center
    );
    assert_eq!(
        serde_json::from_str::<HAlignment>("\"end\"").unwrap(),
        HAlignment::End
    );
}
