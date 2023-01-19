//! Session State Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package applin --example session_state
//! Access the app with an Applin client at http://127.0.0.1:8000/
//! INFO GET / => 200 len=226
//! INFO POST /increment => 200 len=226
//! INFO POST /increment => 200 len=226
//! ```
//!
//! Connect to it, get the session cookie, and call the `/increment` RPC a few times.
//! ```
//! $ curl --include http://127.0.0.1:8000/
//! HTTP/1.1 200 OK
//! content-type: application/json; charset=UTF-8
//! content-length: 226
//! cache-control: no-store
//! set-cookie: session=2623053141802024565-240601519532896979; HttpOnly; Max-Age=2592000; SameSite=Strict
//!
//! {"pages":{"/":{"title":"Session State Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"Counter: 0","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $ curl -X POST http://127.0.0.1:8000/increment --data '' --cookie session=2623053141802024565-240601519532896979
//! {"pages":{"/":{"title":"Session State Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"Counter: 1","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $ curl -X POST http://127.0.0.1:8000/increment --data '' --cookie session=2623053141802024565-240601519532896979
//! {"pages":{"/":{"title":"Session State Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"Counter: 2","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $
//! ```
#![forbid(unsafe_code)]

use applin::action::rpc;
use applin::data::{random_u64, Rebuilder, Roster};
use applin::session::{ApplinSession, PageMap, SessionSet};
use applin::widget::{Button, Column, NavPage, Text};
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{print_log_response, socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
use std::fmt::{Debug, Formatter};
use std::ops::AddAssign;
use std::sync::Arc;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct UserId(pub u64);
impl UserId {
    pub fn new_random() -> Self {
        Self(random_u64())
    }
}
impl Debug for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "UserId({})", self.0)
    }
}

#[derive(Debug)]
struct SessionState {
    #[allow(dead_code)]
    pub user_id: UserId,
    pub count: Roster<u64, Self>,
}
impl SessionState {
    #[must_use]
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            count: Roster::new(0),
        }
    }
}

struct ServerState {
    sessions: SessionSet<SessionState>,
}
impl ServerState {
    pub fn new(executor: &Arc<safina_executor::Executor>) -> Self {
        Self {
            sessions: SessionSet::new(executor),
        }
    }
}

fn page_map(_state: &Arc<ServerState>) -> PageMap<SessionState> {
    let mut keys = PageMap::new();
    keys.add_page_fn("/", move |rebuilder: Rebuilder<SessionState>| {
        let session = rebuilder.session()?;
        let session_state = session.value();
        Ok(NavPage::new(
            "Session State Example",
            Column::new((
                Text::new(format!(
                    "Counter: {}",
                    // Get the counter value and subscribe to updates.
                    // Whenever the value changes, Applin calls
                    // this page function to rebuild the page.
                    *session_state.count.read(rebuilder)
                )),
                Button::new("Increment").with_action(rpc("/increment")),
            )),
        )
        .with_poll(10))
    });
    keys
}

fn get_or_new_session(
    state: &Arc<ServerState>,
    req: &Request,
) -> Result<Arc<ApplinSession<SessionState>>, Response> {
    let state_clone = state.clone();
    state.sessions.get_or_new(
        req,
        move |_rebuilder| Ok(page_map(&state_clone)),
        || SessionState::new(UserId::new_random()),
    )
}

fn increment(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    session
        .value()
        .count
        .write(&session.rpc_context())
        .add_assign(1);
    session.rpc_response()
}

fn handle_req(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    match (req.method(), req.url().path()) {
        ("GET", "/health") => Ok(Response::text(200, "ok")),
        ("GET", "/") => get_or_new_session(state, req)?.poll(),
        ("GET", "/stream") => get_or_new_session(state, req)?.stream(),
        ("POST", "/increment") => increment(state, req),
        _ => Ok(Response::text(404, "Not found")),
    }
}

pub fn main() {
    println!("Access the app with an Applin client at http://127.0.0.1:8000/");
    safina_timer::start_timer_thread();
    let executor = safina_executor::Executor::default();
    let state = Arc::new(ServerState::new(&executor));
    let request_handler = move |req: Request| print_log_response(&req, handle_req(&state, &req));
    executor
        .block_on(
            HttpServerBuilder::new()
                .listen_addr(socket_addr_127_0_0_1(8000))
                .max_conns(100)
                .spawn_and_join(request_handler),
        )
        .unwrap();
}
