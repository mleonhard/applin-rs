mod util;

use applin::data::{Context, Roster};
use applin::session::{KeySet, SessionSet};
use applin::testing::{start_for_test, TestClient};
use applin::widget::{Empty, NavPage, Text};
use serde_json::json;
use servlin::reexport::safina_executor::Executor;
use servlin::reexport::safina_timer;
use servlin::{Request, Response};
use std::ops::{AddAssign, BitXorAssign};
use std::sync::Arc;
use std::time::Duration;

#[test]
pub fn key_set_updates() {
    struct ServerState {
        show_page2: Roster<bool, ()>,
        sessions: SessionSet<()>,
    }
    impl ServerState {
        pub fn new(executor: &Arc<Executor>) -> Self {
            Self {
                show_page2: Roster::new(false).with_cleanup_task(executor),
                sessions: SessionSet::new(executor),
            }
        }
    }
    safina_timer::start_timer_thread();
    let executor = Executor::new(1, 1).unwrap();
    let server_state = Arc::new(ServerState::new(&executor));
    let server_state2 = Arc::clone(&server_state);
    let key_set_fn = move |rebuilder| {
        let mut key_set = KeySet::new();
        key_set.add_static_page("/", NavPage::new("Home", Empty::new()));
        if *server_state2.show_page2.read(rebuilder) {
            key_set.add_static_page("/page2", NavPage::new("Page 2", Empty::new()));
        }
        Ok(key_set)
    };
    let server_state3 = Arc::clone(&server_state);
    let req_handler = move |req: Request| match (req.method.as_str(), req.url.path()) {
        ("GET", "/") => server_state3
            .sessions
            .get_or_new(&req, key_set_fn, || ())?
            .poll(),
        ("GET", "/stream") => server_state3
            .sessions
            .get_or_new(&req, key_set_fn, || ())?
            .stream(),
        ("POST", "/toggle") => {
            let session = server_state3.sessions.get(&req)?;
            server_state3
                .show_page2
                .write(&session.rpc_context())
                .bitxor_assign(true);
            session.rpc_response()
        }
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    let only_home =
        json!({"pages": {"/": {"typ": "nav-page", "title": "Home", "widget": {"typ":"empty"}}}});
    let home_and_page2 = json!({"pages": {
        "/": {"typ": "nav-page", "title": "Home", "widget": {"typ":"empty"}},
        "/page2": {"typ": "nav-page", "title": "Page 2", "widget": {"typ":"empty"}},
    }});
    let empty_update = json!({});
    let add_page2 = json!({"pages": {"/page2": {"typ": "nav-page", "title": "Page 2", "widget": {"typ":"empty"}}}});
    let remove_page2 = json!({"pages": {"/page2": null}});
    let poller1 = TestClient::new(&url);
    assert_eq!(only_home, poller1.poll().unwrap());
    assert_eq!(empty_update, poller1.poll().unwrap());
    // Background thread updates state.
    *server_state.show_page2.write(&Context::Empty) = true;
    assert_eq!(home_and_page2, TestClient::new(&url).poll().unwrap());
    assert_eq!(add_page2, poller1.poll().unwrap());
    assert_eq!(empty_update, poller1.poll().unwrap());
    // RPC updates state.
    let poller2 = TestClient::new(&url);
    poller2.poll().unwrap();
    let streamer = TestClient::new(&url);
    let messages = streamer.stream().unwrap();
    assert_eq!(
        &remove_page2,
        &poller1.post_json("/toggle", json!({})).unwrap()
    );
    assert_eq!(&remove_page2, &poller2.poll().unwrap());
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(
        &[home_and_page2, remove_page2],
        messages.pop_all().as_slice()
    );
}

#[test]
pub fn page_updates() {
    struct ServerState {
        counter: Roster<u32, ()>,
        sessions: SessionSet<()>,
    }
    impl ServerState {
        pub fn new(executor: &Arc<Executor>) -> Self {
            Self {
                counter: Roster::new(3).with_cleanup_task(executor),
                sessions: SessionSet::new(executor),
            }
        }
    }
    safina_timer::start_timer_thread();
    let executor = Executor::new(1, 1).unwrap();
    let server_state = Arc::new(ServerState::new(&executor));
    let server_state2 = Arc::clone(&server_state);
    let key_set_fn = move |_| {
        let server_state3 = Arc::clone(&server_state2);
        Ok(KeySet::new().with_page_fn("/", move |rebuilder| {
            Ok(NavPage::new(
                "t1",
                Text::new(format!("count: {}", server_state3.counter.read(rebuilder))),
            ))
        }))
    };
    let server_state4 = Arc::clone(&server_state);
    let req_handler = move |req: Request| match (req.method.as_str(), req.url.path()) {
        ("GET", "/") => server_state4
            .sessions
            .get_or_new(&req, key_set_fn, || ())?
            .poll(),
        ("GET", "/stream") => server_state4
            .sessions
            .get_or_new(&req, key_set_fn, || ())?
            .stream(),
        ("POST", "/increment") => {
            let session = server_state4.sessions.get(&req)?;
            server_state4
                .counter
                .write(&session.rpc_context())
                .add_assign(1);
            session.rpc_response()
        }
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    let poller1 = TestClient::new(&url);
    let update3 = json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "count: 3"}}}});
    assert_eq!(update3, poller1.poll().unwrap());
    let empty_update = json!({});
    assert_eq!(empty_update, poller1.poll().unwrap());
    // Background thread updates state.
    *server_state.counter.write(&Context::Empty) = 5;
    let update5 = json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "count: 5"}}}});
    assert_eq!(update5, TestClient::new(&url).poll().unwrap());
    assert_eq!(update5, poller1.poll().unwrap());
    assert_eq!(empty_update, poller1.poll().unwrap());
    // RPC updates state.
    let poller2 = TestClient::new(&url);
    poller2.poll().unwrap();
    let streamer = TestClient::new(&url);
    let messages = streamer.stream().unwrap();
    let update6 = json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "count: 6"}}}});
    assert_eq!(
        &update6,
        &poller1.post_json("/increment", json!({})).unwrap()
    );
    assert_eq!(&update6, &poller2.poll().unwrap());
    std::thread::sleep(Duration::from_millis(100));
    assert_eq!(&[update5, update6], messages.pop_all().as_slice());
}
