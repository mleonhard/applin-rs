//! Clock Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package maggie --example clock
//! Access the app with a Maggie client at http://127.0.0.1:8000/
//! INFO GET / => 200 streamed
//! ```
//!
//! Connect one client:
//! ```
//! $ curl http://127.0.0.1:8000/
//! data: {"pages":{"/":{"title":"Clock Example","typ":"nav-page","widget":{"text":"elapsed: 20","typ":"text"}}}}
//! data: {"pages":{"/":{"title":"Clock Example","typ":"nav-page","widget":{"text":"elapsed: 21","typ":"text"}}}}
//! data: {"pages":{"/":{"title":"Clock Example","typ":"nav-page","widget":{"text":"elapsed: 22","typ":"text"}}}}
//! data: {"pages":{"/":{"title":"Clock Example","typ":"nav-page","widget":{"text":"elapsed: 23","typ":"text"}}}}
//! ^C
//! ```
#![forbid(unsafe_code)]

use beatrice::reexport::{safina_executor, safina_timer};
use beatrice::{print_log_response, socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
use maggie::builder::{NavPage, Text};
use maggie::data::{Context, Roster};
use maggie::page::KeySet;
use maggie::session::SessionSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct SessionState {}

struct ServerState {
    displayed_string: Roster<String, SessionState>,
    sessions: SessionSet<SessionState>,
}
impl ServerState {
    pub fn new(executor: &Arc<safina_executor::Executor>) -> Self {
        Self {
            displayed_string: Roster::new(String::new()).with_cleanup_task(executor),
            sessions: SessionSet::new(executor),
        }
    }
}

fn key_set(state: &Arc<ServerState>) -> KeySet<SessionState> {
    let mut keys = KeySet::new();
    let state_clone = state.clone();
    keys.add_page_fn("/", move |ctx| {
        Ok(NavPage::new(
            "Clock Example",
            Text::new(
                // Get the string and subscribe to updates.
                // Whenever the value changes, the server rebuilds this key
                // and pushes it to the client.
                state_clone.displayed_string.read(ctx).to_string(),
            ),
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
    Ok(response)
}

fn handle_req(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    match (req.method(), req.url().path()) {
        ("GET", "/health") => Ok(Response::text(200, "ok")),
        ("GET", "/") => connect(state, req),
        _ => Ok(Response::text(404, "Not found")),
    }
}

pub fn main() {
    println!("Access the app with a Maggie client at http://127.0.0.1:8000/");
    safina_timer::start_timer_thread();
    let executor = safina_executor::Executor::default();
    let state = Arc::new(ServerState::new(&executor));
    let state_clone = state.clone();
    let start = Instant::now();
    std::thread::spawn(move || loop {
        let elapsed = Instant::now() - start;
        let new_string = format!("elapsed: {}", elapsed.as_secs());
        *state_clone.displayed_string.write(&Context::Empty) = new_string;
        let nanos_to_sleep = 1_000_000_000 - elapsed.as_nanos() % 1_000_000_000;
        std::thread::sleep(Duration::from_nanos(u64::try_from(nanos_to_sleep).unwrap()));
    });
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
