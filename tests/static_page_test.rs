#![allow(clippy::missing_panics_doc)]
mod util;

use applin::data::Rebuilder;
use applin::session::{PageMap, SessionSet};
use applin::widget::{NavPage, Text};
use serde_json::json;
use servlin::reexport::safina_executor::Executor;
use servlin::{Request, Response};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use util::{start_for_test, TestClient};

#[test]
pub fn static_page() {
    let executor = Executor::new(1, 1).unwrap();
    let sessions = Arc::new(SessionSet::new(&executor));
    let page_map_fn =
        |_| Ok(PageMap::new().with_static_page("/", NavPage::new("t1", Text::new("hello1"))));
    let req_handler = move |req: Request| match (req.method.as_str(), req.url.path()) {
        ("GET", "/") => sessions.get_or_new(&req, page_map_fn, || ())?.poll(),
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    assert_eq!(
        json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "hello1"}}}}),
        TestClient::new(&url).poll().unwrap()
    );
    assert_eq!(
        Err((404, "Not Found".to_string())),
        TestClient::new(&url).get_json("/nonexistent")
    );
    // TODO: Uncomment test and fix bug.
    // assert_eq!(
    //     404,
    //     new_agent()
    //         .get(url.clone() + "//nonexistent")
    //         .call()
    //         .unwrap_err()
    //         .unwrap_status()
    //         .0
    // );
}

#[test]
pub fn user_specific_static_page() {
    static ID_COUNTER: AtomicU32 = AtomicU32::new(3);
    struct UserState {
        id: u32,
    }
    impl UserState {
        fn new() -> Self {
            Self {
                id: ID_COUNTER.fetch_add(1, Ordering::AcqRel),
            }
        }
    }
    let executor = Executor::new(1, 1).unwrap();
    let sessions: Arc<SessionSet<UserState>> = Arc::new(SessionSet::new(&executor));
    let req_handler = move |req: Request| match (req.method.as_str(), req.url.path()) {
        ("GET", "/") => {
            let page_map_fn = |rebuilder: Rebuilder<UserState>| {
                let id = rebuilder.session()?.state().id;
                Ok(PageMap::new()
                    .with_static_page("/", NavPage::new("t1", Text::new(format!("hello {}", id)))))
            };
            sessions
                .get_or_new(&req, page_map_fn, UserState::new)?
                .poll()
        }
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    let client1 = TestClient::new(&url);
    let client2 = TestClient::new(&url);
    assert_eq!(
        json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "hello 3"}}}}),
        client1.poll().unwrap()
    );
    assert_eq!(
        json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "hello 4"}}}}),
        client2.poll().unwrap()
    );
    assert_eq!(json!({}), client1.poll().unwrap());
}

#[test]
pub fn user_specific_page_map() {
    static ID_COUNTER: AtomicU32 = AtomicU32::new(3);
    struct UserState {
        id: u32,
    }
    impl UserState {
        fn new() -> Self {
            Self {
                id: ID_COUNTER.fetch_add(1, Ordering::AcqRel),
            }
        }
    }
    let executor = Executor::new(1, 1).unwrap();
    let sessions: Arc<SessionSet<UserState>> = Arc::new(SessionSet::new(&executor));
    let req_handler = move |req: Request| match (req.method.as_str(), req.url.path()) {
        ("GET", "/") => {
            let page_map_fn = |rebuilder: Rebuilder<UserState>| {
                let id = rebuilder.session()?.state().id;
                Ok(PageMap::new().with_static_page(
                    format!("/user{}", id),
                    NavPage::new("t1", Text::new(format!("hello {}", id))),
                ))
            };
            sessions
                .get_or_new(&req, page_map_fn, UserState::new)?
                .poll()
        }
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    let client1 = TestClient::new(&url);
    let client2 = TestClient::new(&url);
    assert_eq!(
        json!({"pages": {"/user3": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "hello 3"}}}}),
        client1.poll().unwrap()
    );
    assert_eq!(
        json!({"pages": {"/user4": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "hello 4"}}}}),
        client2.poll().unwrap()
    );
    assert_eq!(json!({}), client1.poll().unwrap());
}
