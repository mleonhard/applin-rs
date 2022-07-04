use applin::internal::{Action, Widget};
use applin::widget::HAlignment;
use serde_json::{json, Value};

#[test]
fn widget_to_value() {
    assert_eq!(
        Widget::Text {
            text: "abc".to_string()
        }
        .to_value(),
        json!({"typ": "text", "text": "abc"})
    );
}

#[test]
fn value_from_widget() {
    let value: Value = Widget::Text {
        text: "abc".to_string(),
    }
    .into();
    assert_eq!(value, json!({"typ": "text", "text": "abc"}));
}

#[test]
fn widget_default() {
    assert_eq!(Widget::default(), Widget::Empty);
}

#[test]
fn widget_back_button_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::BackButton {
            actions: Vec::new()
        })
        .unwrap(),
        r#"{"typ":"back-button","actions":[]}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::BackButton {
            actions: vec![Action::Pop, Action::Logout]
        })
        .unwrap(),
        r#"{"typ":"back-button","actions":["pop","logout"]}"#
    );
}

#[test]
fn widget_back_button_deserialize() {
    serde_json::from_str::<Widget>(r#"{"typ":"back-button"}"#)
        .expect_err("back-button requires `actions`");
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"back-button","actions":[]}"#).unwrap(),
        Widget::BackButton {
            actions: Vec::new()
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"back-button","actions":["pop","logout"]}"#)
            .unwrap(),
        Widget::BackButton {
            actions: vec![Action::Pop, Action::Logout]
        }
    );
}

#[test]
fn widget_button_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::Button {
            text: "".to_string(),
            actions: Vec::new(),
        })
        .unwrap(),
        r#"{"typ":"button","text":""}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::Button {
            text: "abc".to_string(),
            actions: vec![Action::Pop, Action::Logout],
        })
        .unwrap(),
        r#"{"typ":"button","text":"abc","actions":["pop","logout"]}"#
    );
}

#[test]
fn widget_button_deserialize() {
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"button","text":""}"#).unwrap(),
        Widget::Button {
            text: "".to_string(),
            actions: Vec::new(),
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(
            r#"{"typ":"button","text":"abc","actions":["pop","logout"]}"#
        )
        .unwrap(),
        Widget::Button {
            text: "abc".to_string(),
            actions: vec![Action::Pop, Action::Logout],
        }
    );
}

#[test]
fn widget_column_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::Column {
            widgets: Vec::new(),
            h_alignment: HAlignment::Start,
            spacing: 0
        })
        .unwrap(),
        r#"{"typ":"column","h-alignment":"start"}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::Column {
            widgets: vec![
                Widget::Empty,
                Widget::Text {
                    text: "abc".to_string()
                }
            ],
            h_alignment: HAlignment::Center,
            spacing: 5
        })
        .unwrap(),
        r#"{"typ":"column","h-alignment":"center","spacing":5,"widgets":[{"typ":"empty"},{"typ":"text","text":"abc"}]}"#
    );
}

#[test]
fn widget_column_deserialize() {
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"column"}"#).unwrap(),
        Widget::Column {
            widgets: Vec::new(),
            h_alignment: HAlignment::Start,
            spacing: 0
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(
            r#"{"typ":"column","widgets":[{"typ":"empty"},{"typ":"text","text":"abc"}],"h-alignment":"center","spacing":5}"#
        )
        .unwrap(),
        Widget::Column {
            widgets: vec![
                Widget::Empty,
                Widget::Text {
                    text: "abc".to_string()
                }
            ],
            h_alignment: HAlignment::Center,
            spacing: 5
        }
    );
}

#[test]
fn widget_empty_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::Empty).unwrap(),
        r#"{"typ":"empty"}"#
    );
}

#[test]
fn widget_empty_deserialize() {
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"empty"}"#).unwrap(),
        Widget::Empty
    );
}

#[test]
fn widget_form_detail_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::FormDetail {
            actions: Vec::new(),
            photo_url: None,
            sub_text: None,
            text: "".to_string(),
        })
        .unwrap(),
        r#"{"typ":"form-detail","text":""}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::FormDetail {
            actions: vec![Action::Pop, Action::Logout],
            photo_url: Some("/p1".to_string()),
            sub_text: Some("s1".to_string()),
            text: "t1".to_string(),
        })
        .unwrap(),
        r#"{"typ":"form-detail","text":"t1","sub-text":"s1","photo-url":"/p1","actions":["pop","logout"]}"#
    );
}

#[test]
fn widget_form_detail_deserialize() {
    serde_json::from_str::<Widget>(r#"{"typ":"form-detail"}"#)
        .expect_err("form-detail requires `text`");
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"form-detail","text":""}"#).unwrap(),
        Widget::FormDetail {
            actions: Vec::new(),
            photo_url: None,
            sub_text: None,
            text: "".to_string(),
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(
            r#"{"typ":"form-detail","actions":["pop","logout"],"photo-url":"/p1","sub-text":"s1","text":"t1"}"#
        )
        .unwrap(),
        Widget::FormDetail {
            actions: vec![Action::Pop, Action::Logout],
            photo_url: Some("/p1".to_string()),
            sub_text: Some("s1".to_string()),
            text: "t1".to_string(),
        }
    );
}

#[test]
fn widget_form_section_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::FormSection {
            title: None,
            widgets: Vec::new(),
        })
        .unwrap(),
        r#"{"typ":"form-section"}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::FormSection {
            title: Some("title1".to_string()),
            widgets: vec![
                Widget::Empty,
                Widget::Text {
                    text: "abc".to_string()
                }
            ],
        })
        .unwrap(),
        r#"{"typ":"form-section","title":"title1","widgets":[{"typ":"empty"},{"typ":"text","text":"abc"}]}"#
    );
}

#[test]
fn widget_form_section_deserialize() {
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"form-section"}"#).unwrap(),
        Widget::FormSection {
            title: None,
            widgets: Vec::new(),
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(
            r#"{"typ":"form-section","title":"title1","widgets":[{"typ":"empty"},{"typ":"text","text":"abc"}]}"#
        )
            .unwrap(),
        Widget::FormSection {
            title: Some("title1".to_string()),
            widgets: vec![
                Widget::Empty,
                Widget::Text {
                    text: "abc".to_string()
                }
            ],
        }
    );
}

#[test]
fn widget_modal_button_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::ModalButton {
            text: "".to_string(),
            actions: Vec::new(),
            is_cancel: false,
            is_default: false,
            is_destructive: false
        })
        .unwrap(),
        r#"{"typ":"modal-button","text":""}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::ModalButton {
            text: "abc".to_string(),
            actions: vec![Action::Pop, Action::Logout],
            is_cancel: false,
            is_default: false,
            is_destructive: false
        })
        .unwrap(),
        r#"{"typ":"modal-button","text":"abc","actions":["pop","logout"]}"#
    );
}

#[test]
fn widget_modal_button_deserialize() {
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"modal-button","text":""}"#).unwrap(),
        Widget::ModalButton {
            text: "".to_string(),
            actions: Vec::new(),
            is_cancel: false,
            is_default: false,
            is_destructive: false
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(
            r#"{"typ":"modal-button","text":"abc","actions":["pop","logout"]}"#
        )
        .unwrap(),
        Widget::ModalButton {
            text: "abc".to_string(),
            actions: vec![Action::Pop, Action::Logout],
            is_cancel: false,
            is_default: false,
            is_destructive: false
        }
    );
}

#[test]
fn widget_text_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::Text {
            text: "".to_string()
        })
        .unwrap(),
        r#"{"typ":"text","text":""}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::Text {
            text: "t1".to_string()
        })
        .unwrap(),
        r#"{"typ":"text","text":"t1"}"#
    );
}

#[test]
fn widget_text_deserialize() {
    serde_json::from_str::<Widget>(r#"{"typ":"text"}"#).expect_err("text requires `text`");
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"text","text":""}"#).unwrap(),
        Widget::Text {
            text: "".to_string()
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"text","text":"t1"}"#).unwrap(),
        Widget::Text {
            text: "t1".to_string()
        }
    );
}
