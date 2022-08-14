mod util;

use crate::util::{new_agent, start_for_test, UreqError, UreqJsonHelper};
use applin::data::Rebuilder;
use applin::session::{KeySet, SessionSet};
use applin::widget::{NavPage, Text};
use serde_json::json;
use servlin::reexport::safina_executor::Executor;
use servlin::{Request, Response};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[test]
pub fn static_page() {
    let executor = Executor::new(1, 1).unwrap();
    let sessions = Arc::new(SessionSet::new(&executor));
    let key_set_fn =
        |_| Ok(KeySet::new().with_static_page("/", NavPage::new("t1", Text::new("hello1"))));
    let req_handler = move |req: Request| match (req.method.as_str(), req.url.path()) {
        ("GET", "/") => sessions.get_or_new(&req, key_set_fn, || ())?.poll(),
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    assert_eq!(
        json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "hello1"}}}}),
        new_agent().get_json(&url).unwrap()
    );
    assert_eq!(
        Err(UreqError::Status(404)),
        new_agent().get_json(url.clone() + "/nonexistent")
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
            let key_set_fn = |rebuilder: Rebuilder<UserState>| {
                let id = rebuilder.session()?.state().id;
                Ok(KeySet::new()
                    .with_static_page("/", NavPage::new("t1", Text::new(format!("hello {}", id)))))
            };
            sessions
                .get_or_new(&req, key_set_fn, UserState::new)?
                .poll()
        }
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    let agent1 = new_agent();
    let agent2 = new_agent();
    assert_eq!(
        json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "hello 3"}}}}),
        agent1.get_json(&url).unwrap()
    );
    assert_eq!(
        json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "hello 4"}}}}),
        agent2.get_json(&url).unwrap()
    );
    assert_eq!(json!({}), agent1.get_json(&url).unwrap());
}

#[test]
pub fn user_specific_key_set() {
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
            let key_set_fn = |rebuilder: Rebuilder<UserState>| {
                let id = rebuilder.session()?.state().id;
                Ok(KeySet::new().with_static_page(
                    format!("/user{}", id),
                    NavPage::new("t1", Text::new(format!("hello {}", id))),
                ))
            };
            sessions
                .get_or_new(&req, key_set_fn, UserState::new)?
                .poll()
        }
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    let agent1 = new_agent();
    let agent2 = new_agent();
    assert_eq!(
        json!({"pages": {"/user3": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "hello 3"}}}}),
        agent1.get_json(&url).unwrap()
    );
    assert_eq!(
        json!({"pages": {"/user4": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "hello 4"}}}}),
        agent2.get_json(&url).unwrap()
    );
    assert_eq!(json!({}), agent1.get_json(&url).unwrap());
}
