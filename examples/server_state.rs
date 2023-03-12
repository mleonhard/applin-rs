//! Server State Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package applin --example server_state
//! Access the app with an Applin client at http://127.0.0.1:8000/
//! Access the app with an Applin client at http://127.0.0.1:8000/
//! INFO GET / => 200 len=239
//! INFO GET /stream => 200 streamed
//! INFO GET / => 200 len=239
//! INFO POST /increment => 200 len=239
//! INFO POST /increment => 200 len=239
//! ```
//!
//! Connect one client:
//! ```
//! $ curl http://127.0.0.1:8000/
//! {"pages":{"/":{"stream":true,"title":"Server State Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"Counter: 0","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $ curl http://127.0.0.1:8000/stream
//! data: {"pages":{"/":{"stream":true,"title":"Server State Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"Counter: 0","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! data: {"pages":{"/":{"stream":true,"title":"Server State Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"Counter: 1","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! data: {"pages":{"/":{"stream":true,"title":"Server State Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"Counter: 2","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! ^C
//! ```
//!
//! Connect another client and call the `/increment` RPC twice.
//! The first client will immediately receive updates.
//! ```
//! $ curl --include http://127.0.0.1:8000/
//! HTTP/1.1 200 OK
//! content-type: application/json; charset=UTF-8
//! content-length: 239
//! cache-control: no-store
//! set-cookie: session=10710489863702200797-4063056143104496287; HttpOnly; Max-Age=2592000; SameSite=Strict
//!
//! {"pages":{"/":{"stream":true,"title":"Server State Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"Counter: 0","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $ curl http://127.0.0.1:8000/increment -X POST -d '' --cookie session=10710489863702200797-4063056143104496287
//! {"pages":{"/":{"stream":true,"title":"Server State Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"Counter: 1","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $ curl http://127.0.0.1:8000/increment -X POST -d '' --cookie session=10710489863702200797-4063056143104496287
//! {"pages":{"/":{"stream":true,"title":"Server State Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"Counter: 2","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $
//! ```
//!
#![forbid(unsafe_code)]

use applin::action::rpc;
use applin::data::Roster;
use applin::session::{ApplinSession, PageMap, SessionSet};
use applin::widget::{Column, FormButton, NavPage, Text};
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{print_log_response, socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
use std::ops::AddAssign;
use std::sync::Arc;

struct SessionState {}

struct ServerState {
    counter: Roster<u64, SessionState>,
    sessions: SessionSet<SessionState>,
}
impl ServerState {
    pub fn new(executor: &Arc<safina_executor::Executor>) -> Self {
        Self {
            counter: Roster::new(0).with_cleanup_task(executor),
            sessions: SessionSet::new(executor),
        }
    }
}

fn page_map(state: &Arc<ServerState>) -> PageMap<SessionState> {
    let mut keys = PageMap::new();
    let state_clone = state.clone();
    keys.add_page_fn("/", move |rebuilder| {
        Ok(NavPage::new(
            "Server State Example",
            Column::new((
                Text::new(format!(
                    "Counter: {}",
                    // Get the counter value and subscribe to updates.
                    // Whenever the value changes, the server rebuilds this key
                    // and pushes it to the client.
                    *state_clone.counter.read(rebuilder)
                )),
                FormButton::new("Increment").with_action(rpc("/increment")),
            )),
        )
        .with_stream())
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
        || SessionState {},
    )
}

fn increment(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    //dbg!(req);
    let session = state.sessions.get(req)?;
    state.counter.write(session.rpc_context()).add_assign(1);
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
