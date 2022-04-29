//! Session State Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package maggie --example session_state
//! Access the app with a Maggie client at http://127.0.0.1:8000/
//! INFO GET / => 200 streamed
//! INFO POST /increment => 200 len=204
//! INFO POST /increment => 200 len=204
//! ```
//!
//! Connect to it, get the session cookie, and call the `/increment` RPC a few times.
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
//! < set-cookie: session=2623053141802024565-240601519532896979; HttpOnly; Max-Age=2592000; SameSite=Strict; Secure
//! <
//! data: {"pages":{"/":{"title":"Session State Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"Counter: 0","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! ^C
//! $ curl -X POST http://127.0.0.1:8000/increment --data '' --cookie session=2623053141802024565-240601519532896979
//! {"pages":{"/":{"title":"Session State Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"Counter: 1","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $ curl -X POST http://127.0.0.1:8000/increment --data '' --cookie session=2623053141802024565-240601519532896979
//! {"pages":{"/":{"title":"Session State Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"Counter: 2","typ":"text"},{"actions":["rpc:/increment"],"text":"Increment","typ":"button"}]}}}}
//! $
//! ```
#![forbid(unsafe_code)]

use beatrice::reexport::{safina_executor, safina_timer};
use beatrice::{print_log_response, socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
use maggie::context::Context;
use maggie::key_set::KeySet;
use maggie::pages::NavPage;
use maggie::random::random_u64;
use maggie::roster::Roster;
use maggie::session_set::SessionSet;
use maggie::widgets::{Button, Column, Text};
use std::error::Error;
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

#[allow(clippy::unnecessary_wraps)]
fn key_set(
    _state: &Arc<ServerState>,
    _ctx: &Context<SessionState>,
) -> Result<KeySet<SessionState>, Box<dyn Error>> {
    let mut keys = KeySet::new();
    keys.add_page_fn("/", move |ctx: &Context<SessionState>| {
        Ok(NavPage::new(
            "Session State Example",
            Column::new((
                Text::new(format!(
                    "Counter: {}",
                    // Get the counter value and subscribe to updates.
                    // Whenever the value changes, the server rebuilds this key
                    // and pushes it to the client.
                    *ctx.session()?.state().count.read(ctx)
                )),
                Button::new("Increment").with_action("rpc:/increment"),
            )),
        ))
    });
    Ok(keys)
}

fn connect(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let state_clone = state.clone();
    let (_session, response) = state.sessions.resume_or_new(
        req,
        move |ctx| key_set(&state_clone, ctx),
        || SessionState::new(UserId::new_random()),
    )?;
    Ok(response)
}

fn increment(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    session
        .state()
        .count
        .write(&session.rpc_context())
        .add_assign(1);
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
