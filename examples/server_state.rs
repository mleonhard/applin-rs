//! Server State Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package maggie --example server_state
//! Access the app with a Maggie client at http://127.0.0.1:8000/
//! INFO GET / => 200 streamed
//! INFO GET / => 200 streamed
//! INFO POST /increment => 200 len=203
//! INFO POST /increment => 200 len=203
//! ```
//!
//! Connect one client:
//! ```
//! $ curl http://127.0.0.1:8000/
//! data: {"pages":{"/":{"title":"Server State Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"Counter: 0","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! data: {"pages":{"/":{"title":"Server State Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"Counter: 1","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! data: {"pages":{"/":{"title":"Server State Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"Counter: 2","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! ^C
//! ```
//!
//! Connect another client and press CTRL-C.  Then call the `/increment` RPC twice.
//! The first client will immediately receive updates.
//! ```
//! $ curl -v http://127.0.0.1:8000/
//! *   Trying 127.0.0.1...
//! * TCP_NODELAY set
//! * Connected to 127.0.0.1 (127.0.0.1) port 8000 (#0)
//! > GET / HTTP/1.1
//! > Host: 127.0.0.1:8000
//! > User-Agent: curl/7.64.1
//! > Accept: */*
//! >
//! < HTTP/1.1 200 OK
//! < content-type: text/event-stream
//! < transfer-encoding: chunked
//! < set-cookie: session=10710489863702200797-4063056143104496287; HttpOnly; Max-Age=2592000; SameSite=Strict; Secure
//! <
//! data: {"pages":{"/":{"title":"Server State Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"Counter: 0","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! ^C
//! $ curl http://127.0.0.1:8000/increment -X POST -d '' --cookie session=10710489863702200797-4063056143104496287
//! {"pages":{"/":{"title":"Server State Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"Counter: 1","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $ curl http://127.0.0.1:8000/increment -X POST -d '' --cookie session=10710489863702200797-4063056143104496287
//! {"pages":{"/":{"title":"Server State Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"Counter: 2","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $
//! ```
//!
#![forbid(unsafe_code)]

use beatrice::reexport::{safina_executor, safina_timer};
use beatrice::{print_log_response, socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
use maggie::key_set::KeySet;
use maggie::pages::NavPage;
use maggie::roster::Roster;
use maggie::session_set::SessionSet;
use maggie::widgets::{Button, Column, Text};
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

fn key_set(state: &Arc<ServerState>) -> KeySet<SessionState> {
    let mut keys = KeySet::new();
    let state_clone = state.clone();
    keys.add_page_fn("/", move |ctx| {
        Ok(NavPage::new(
            "Server State Example",
            Column::new((
                Text::new(format!(
                    "Counter: {}",
                    // Get the counter value and subscribe to updates.
                    // Whenever the value changes, the server rebuilds this key
                    // and pushes it to the client.
                    *state_clone.counter.read(ctx)
                )),
                Button::new("Increment").with_action("rpc:/increment"),
            )),
        ))
    });
    keys
}

fn connect(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let state_clone = state.clone();
    let (_session, response) = state.sessions.resume_or_new(
        req,
        move |_ctx| Ok(key_set(&state_clone)),
        || SessionState {},
    )?;
    //dbg!(&response);
    Ok(response)
}

fn increment(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    //dbg!(req);
    let session = state.sessions.get(req)?;
    state.counter.write(&session.rpc_context()).add_assign(1);
    session.rpc_response()
}

fn handle_req(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    match (req.method(), req.url().path()) {
        ("GET", "/health") => Ok(Response::text(200, "ok")),
        ("GET", "/") => connect(state, req),
        ("POST", "/increment") => increment(state, req),
        _ => Ok(Response::text(404, "Not found")),
    }
}

pub fn main() {
    println!("Access the app with a Maggie client at http://127.0.0.1:8000/");
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
