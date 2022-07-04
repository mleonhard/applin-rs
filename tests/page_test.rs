use applin::internal::{Page, Widget};
use serde_json::{json, Value};

#[test]
fn page_to_value() {
    assert_eq!(
        Page::Plain {
            poll_seconds: 0,
            stream: false,
            title: "T1".to_string(),
            widget: Widget::Empty
        }
        .to_value(),
        json!({"typ": "plain-page", "title": "T1", "widget": { "typ":"empty" }})
    );
}

#[test]
fn value_from_page() {
    let value: Value = Page::Plain {
        poll_seconds: 0,
        stream: false,
        title: "T1".to_string(),
        widget: Widget::Empty,
    }
    .into();
    assert_eq!(
        value,
        json!({"typ": "plain-page", "title": "T1", "widget": { "typ":"empty" }})
    );
}

#[test]
fn page_default() {
    assert_eq!(
        Page::default().to_value(),
        json!({"typ": "nav-page", "title": "Default Page", "widget": { "typ":"empty" }})
    );
}

#[test]
fn page_nav_serialize() {
    assert_eq!(
        serde_json::to_string(&Page::Nav {
            end: None,
            poll_seconds: 0,
            start: None,
            stream: false,
            title: "".to_string(),
            widget: Widget::Empty,
        })
        .unwrap(),
        r#"{"typ":"nav-page","title":"","widget":{"typ":"empty"}}"#
    );
    assert_eq!(
        serde_json::to_string(&Page::Nav {
            end: Some(Widget::Text {
                text: "e1".to_string()
            }),
            poll_seconds: 0,
            start: Some(Widget::Text {
                text: "s1".to_string()
            }),
            stream: false,
            title: "T1".to_string(),
            widget: Widget::Text {
                text: "w1".to_string()
            },
        })
        .unwrap(),
        r#"{"typ":"nav-page","end":{"typ":"text","text":"e1"},"start":{"typ":"text","text":"s1"},"title":"T1","widget":{"typ":"text","text":"w1"}}"#
    );
}

#[test]
fn page_nav_deserialize() {
    serde_json::from_str::<Page>(r#"{"typ":"nav-page","widget":{"typ":"empty"}}"#)
        .expect_err("nav-page requires `title`");
    serde_json::from_str::<Page>(r#"{"typ":"nav-page","title":""}"#)
        .expect_err("nav-page requires `widget`");
    assert_eq!(
        serde_json::from_str::<Page>(r#"{"typ":"nav-page","title":"","widget":{"typ":"empty"}}"#)
            .unwrap(),
        Page::Nav {
            end: None,
            poll_seconds: 0,
            start: None,
            stream: false,
            title: "".to_string(),
            widget: Widget::Empty,
        }
    );
    assert_eq!(
        serde_json::from_str::<Page>(
            r#"{"typ":"nav-page","title":"T1","widget":{"typ":"text","text":"w1"},"start":{"typ":"text","text":"s1"},"end":{"typ":"text","text":"e1"}}"#
        ).unwrap(),
        Page::Nav {
            end: Some(Widget::Text {
                text: "e1".to_string()
            }),
            poll_seconds: 0,
            start: Some(Widget::Text {
                text: "s1".to_string()
            }),
            stream: false,
            title: "T1".to_string(),
            widget: Widget::Text {
                text: "w1".to_string()
            },
        }
    );
}

#[test]
fn page_plain_serialize() {
    assert_eq!(
        serde_json::to_string(&Page::Plain {
            poll_seconds: 0,
            stream: false,
            title: "".to_string(),
            widget: Widget::Empty,
        })
        .unwrap(),
        r#"{"typ":"plain-page","title":"","widget":{"typ":"empty"}}"#
    );
    assert_eq!(
        serde_json::to_string(&Page::Plain {
            poll_seconds: 0,
            stream: false,
            title: "T1".to_string(),
            widget: Widget::Text {
                text: "w1".to_string()
            },
        })
        .unwrap(),
        r#"{"typ":"plain-page","title":"T1","widget":{"typ":"text","text":"w1"}}"#
    );
}

#[test]
fn page_plain_deserialize() {
    serde_json::from_str::<Page>(r#"{"typ":"plain-page","widget":{"typ":"empty"}}"#)
        .expect_err("plain-page requires `title`");
    serde_json::from_str::<Page>(r#"{"typ":"plain-page","title":""}"#)
        .expect_err("plain-page requires `widget`");
    assert_eq!(
        serde_json::from_str::<Page>(r#"{"typ":"plain-page","title":"","widget":{"typ":"empty"}}"#)
            .unwrap(),
        Page::Plain {
            poll_seconds: 0,
            stream: false,
            title: "".to_string(),
            widget: Widget::Empty,
        }
    );
    assert_eq!(
        serde_json::from_str::<Page>(
            r#"{"typ":"plain-page","title":"T1","widget":{"typ":"text","text":"w1"}}"#
        )
        .unwrap(),
        Page::Plain {
            poll_seconds: 0,
            stream: false,
            title: "T1".to_string(),
            widget: Widget::Text {
                text: "w1".to_string()
            },
        }
    );
}
