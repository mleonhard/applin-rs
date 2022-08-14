mod util;

use crate::util::{new_agent, start_for_test, UreqJsonHelper};
use applin::data::{Context, Roster};
use applin::session::{KeySet, SessionSet};
use applin::widget::{NavPage, Text};
use serde_json::json;
use servlin::reexport::safina_executor::Executor;
use servlin::reexport::safina_timer;
use servlin::{Request, Response};
use std::ops::AddAssign;
use std::sync::Arc;

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

#[test]
pub fn page_updates() {
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
    let poller1 = new_agent();
    // Check check intial page.
    let update3 = json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "count: 3"}}}});
    assert_eq!(update3, poller1.get_json(&url).unwrap());
    let empty_update = json!({});
    assert_eq!(empty_update, poller1.get_json(&url).unwrap());
    // Background thread updates state.
    *server_state.counter.write(&Context::Empty) = 5;
    let update5 = json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "count: 5"}}}});
    assert_eq!(update5, new_agent().get_json(&url).unwrap());
    assert_eq!(update5, poller1.get_json(&url).unwrap());
    assert_eq!(empty_update, poller1.get_json(&url).unwrap());
    // RPC updates state.
    let poller2 = new_agent();
    poller2.get_json(&url).unwrap();
    let streamer = new_agent();
    streamer.get_json(&url).unwrap();
    // let messages = streamer.stream().unwrap();
    let update6 = json!({"pages": {"/": {"typ": "nav-page", "title": "t1", "widget": {"typ":"text", "text": "count: 6"}}}});
    assert_eq!(
        &update6,
        &poller1
            .post_json(url.clone() + "/increment", json!({}))
            .unwrap()
    );
    assert_eq!(&update6, &poller2.get_json(&url).unwrap());
    // Test streaming.
}
