use maggie::widget::Action;
use serde_json::Value;

#[test]
fn action_to_value() {
    assert_eq!(Action::Pop.to_value(), Value::String("pop".to_string()));
}

#[test]
fn value_from_action() {
    let value: Value = Action::Pop.into();
    assert_eq!(value, Value::String("pop".to_string()));
}

#[test]
fn action_serialize() {
    assert_eq!(
        serde_json::to_string(&Action::CopyToClipboard("".to_string())).unwrap(),
        "\"copy-to-clipboard:\""
    );
    assert_eq!(
        serde_json::to_string(&Action::CopyToClipboard("abc".to_string())).unwrap(),
        "\"copy-to-clipboard:abc\""
    );
    assert_eq!(
        serde_json::to_string(&Action::LaunchUrl("".to_string())).unwrap(),
        "\"launch-url:\""
    );
    assert_eq!(
        serde_json::to_string(&Action::LaunchUrl("scheme://path".to_string())).unwrap(),
        "\"launch-url:scheme://path\""
    );
    assert_eq!(
        serde_json::to_string(&Action::Logout).unwrap(),
        "\"logout\""
    );
    assert_eq!(
        serde_json::to_string(&Action::Nothing).unwrap(),
        "\"nothing\""
    );
    assert_eq!(serde_json::to_string(&Action::Pop).unwrap(), "\"pop\"");
    assert_eq!(
        serde_json::to_string(&Action::Push("".to_string())).unwrap(),
        "\"push:\""
    );
    assert_eq!(
        serde_json::to_string(&Action::Push("/page1".to_string())).unwrap(),
        "\"push:/page1\""
    );
    assert_eq!(
        serde_json::to_string(&Action::Rpc("".to_string())).unwrap(),
        "\"rpc:\""
    );
    assert_eq!(
        serde_json::to_string(&Action::Rpc("/method?arg=val".to_string())).unwrap(),
        "\"rpc:/method?arg=val\""
    );
}

#[test]
fn action_deserialize() {
    assert_eq!(
        serde_json::from_str::<Action>("\"copy-to-clipboard:abc:123\"").unwrap(),
        Action::CopyToClipboard("abc:123".to_string())
    );
    assert_eq!(
        serde_json::from_str::<Action>("\"copy-to-clipboard:\"").unwrap(),
        Action::CopyToClipboard("".to_string())
    );
    serde_json::from_str::<Action>("\"launch-url:\"").unwrap_err();
    assert_eq!(
        serde_json::from_str::<Action>("\"launch-url:scheme://path\"").unwrap(),
        Action::LaunchUrl("scheme://path".to_string())
    );
    assert_eq!(
        serde_json::from_str::<Action>("\"logout\"").unwrap(),
        Action::Logout
    );
    assert_eq!(
        serde_json::from_str::<Action>("\"nothing\"").unwrap(),
        Action::Nothing
    );
    assert_eq!(
        serde_json::from_str::<Action>("\"pop\"").unwrap(),
        Action::Pop
    );
    serde_json::from_str::<Action>("\"push:\"").unwrap_err();
    assert_eq!(
        serde_json::from_str::<Action>("\"push:/page1\"").unwrap(),
        Action::Push("/page1".to_string())
    );
    serde_json::from_str::<Action>("\"rpc:\"").unwrap_err();
    assert_eq!(
        serde_json::from_str::<Action>("\"rpc:/method?arg=val\"").unwrap(),
        Action::Rpc("/method?arg=val".to_string())
    );
}
