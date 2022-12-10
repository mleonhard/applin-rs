use applin::internal::{Action, Widget};
use applin::widget::HAlignment;
use serde_json::{json, Value};

#[test]
fn widget_to_value() {
    assert_eq!(
        Widget::TextVariant {
            text: "abc".to_string()
        }
        .to_value(),
        json!({"typ": "text", "text": "abc"})
    );
}

#[test]
fn value_from_widget() {
    let value: Value = Widget::TextVariant {
        text: "abc".to_string(),
    }
    .into();
    assert_eq!(value, json!({"typ": "text", "text": "abc"}));
}

#[test]
fn widget_default() {
    assert_eq!(Widget::default(), Widget::EmptyVariant);
}

#[test]
fn widget_back_button_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::BackButtonVariant {
            actions: Vec::new()
        })
        .unwrap(),
        r#"{"typ":"back-button","actions":[]}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::BackButtonVariant {
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
        Widget::BackButtonVariant {
            actions: Vec::new()
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"back-button","actions":["pop","logout"]}"#)
            .unwrap(),
        Widget::BackButtonVariant {
            actions: vec![Action::Pop, Action::Logout]
        }
    );
}

#[test]
fn widget_button_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::ButtonVariant {
            text: "".to_string(),
            actions: Vec::new(),
        })
        .unwrap(),
        r#"{"typ":"button","text":""}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::ButtonVariant {
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
        Widget::ButtonVariant {
            text: "".to_string(),
            actions: Vec::new(),
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(
            r#"{"typ":"button","text":"abc","actions":["pop","logout"]}"#
        )
        .unwrap(),
        Widget::ButtonVariant {
            text: "abc".to_string(),
            actions: vec![Action::Pop, Action::Logout],
        }
    );
}

#[test]
fn widget_column_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::ColumnVariant {
            widgets: Vec::new(),
            h_alignment: HAlignment::Start,
            spacing: 0
        })
        .unwrap(),
        r#"{"typ":"column","h-alignment":"start"}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::ColumnVariant {
            widgets: vec![
                Widget::EmptyVariant,
                Widget::TextVariant {
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
        Widget::ColumnVariant {
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
        Widget::ColumnVariant {
            widgets: vec![
                Widget::EmptyVariant,
                Widget::TextVariant {
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
        serde_json::to_string(&Widget::EmptyVariant).unwrap(),
        r#"{"typ":"empty"}"#
    );
}

#[test]
fn widget_empty_deserialize() {
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"empty"}"#).unwrap(),
        Widget::EmptyVariant
    );
}

#[test]
fn widget_nav_button_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::NavButtonVariant {
            actions: Vec::new(),
            photo_url: None,
            sub_text: None,
            text: "".to_string(),
        })
        .unwrap(),
        r#"{"typ":"nav-button","text":""}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::NavButtonVariant {
            actions: vec![Action::Pop, Action::Logout],
            photo_url: Some("/p1".to_string()),
            sub_text: Some("s1".to_string()),
            text: "t1".to_string(),
        })
        .unwrap(),
        r#"{"typ":"nav-button","text":"t1","sub-text":"s1","photo-url":"/p1","actions":["pop","logout"]}"#
    );
}

#[test]
fn widget_nav_button_deserialize() {
    serde_json::from_str::<Widget>(r#"{"typ":"nav-button"}"#)
        .expect_err("nav-button requires `text`");
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"nav-button","text":""}"#).unwrap(),
        Widget::NavButtonVariant {
            actions: Vec::new(),
            photo_url: None,
            sub_text: None,
            text: "".to_string(),
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(
            r#"{"typ":"nav-button","actions":["pop","logout"],"photo-url":"/p1","sub-text":"s1","text":"t1"}"#
        )
        .unwrap(),
        Widget::NavButtonVariant {
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
        serde_json::to_string(&Widget::FormSectionVariant {
            title: None,
            widgets: Vec::new(),
        })
        .unwrap(),
        r#"{"typ":"form-section"}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::FormSectionVariant {
            title: Some("title1".to_string()),
            widgets: vec![
                Widget::EmptyVariant,
                Widget::TextVariant {
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
        Widget::FormSectionVariant {
            title: None,
            widgets: Vec::new(),
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(
            r#"{"typ":"form-section","title":"title1","widgets":[{"typ":"empty"},{"typ":"text","text":"abc"}]}"#
        )
            .unwrap(),
        Widget::FormSectionVariant {
            title: Some("title1".to_string()),
            widgets: vec![
                Widget::EmptyVariant,
                Widget::TextVariant {
                    text: "abc".to_string()
                }
            ],
        }
    );
}

#[test]
fn widget_modal_button_serialize() {
    assert_eq!(
        serde_json::to_string(&Widget::ModalButtonVariant {
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
        serde_json::to_string(&Widget::ModalButtonVariant {
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
        Widget::ModalButtonVariant {
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
        Widget::ModalButtonVariant {
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
        serde_json::to_string(&Widget::TextVariant {
            text: "".to_string()
        })
        .unwrap(),
        r#"{"typ":"text","text":""}"#
    );
    assert_eq!(
        serde_json::to_string(&Widget::TextVariant {
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
        Widget::TextVariant {
            text: "".to_string()
        }
    );
    assert_eq!(
        serde_json::from_str::<Widget>(r#"{"typ":"text","text":"t1"}"#).unwrap(),
        Widget::TextVariant {
            text: "t1".to_string()
        }
    );
}
