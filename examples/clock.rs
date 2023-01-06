//! Clock Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package applin --example clock
//! Access the app with an Applin client at http://127.0.0.1:8000/
//! INFO GET / => 200 len=116
//! ```
//!
//! Connect one client:
//! ```
//! $ curl --include http://127.0.0.1:8000/
//! HTTP/1.1 200 OK
//! content-type: application/json; charset=UTF-8
//! content-length: 117
//! cache-control: no-store
//! set-cookie: session=13048651617783089387-5269282997511680177; HttpOnly; Max-Age=2592000; SameSite=Strict
//!
//! {"pages":{"/":{"stream":true,"title":"Clock Example","typ":"nav-page","widget":{"text":"elapsed: 69","typ":"text"}}}}
//! $
//! ```
#![forbid(unsafe_code)]

use applin::data::{Context, Roster};
use applin::session::{ApplinSession, PageMap, SessionSet};
use applin::widget::{NavPage, Scroll, Text};
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{print_log_response, socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
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

fn page_map(state: &Arc<ServerState>) -> PageMap<SessionState> {
    let mut keys = PageMap::new();
    let state_clone = state.clone();
    keys.add_page_fn("/", move |rebuilder| {
        Ok(NavPage::new(
            "Clock Example",
            Scroll::new(Text::new(
                // Get the string and subscribe to updates.
                // Whenever the value changes, the server rebuilds this key
                // and pushes it to the client.
                state_clone.displayed_string.read(rebuilder).to_string()
                    + "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n",
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

fn handle_req(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    match (req.method(), req.url().path()) {
        ("GET", "/health") => Ok(Response::text(200, "ok")),
        ("GET", "/") => get_or_new_session(state, req)?.poll(),
        ("GET", "/stream") => get_or_new_session(state, req)?.stream(),
        _ => Ok(Response::text(404, "Not found")),
    }
}

pub fn main() {
    println!("Access the app with an Applin client at http://127.0.0.1:8000/");
    safina_timer::start_timer_thread();
    let executor = safina_executor::Executor::default();
    let state = Arc::new(ServerState::new(&executor));
    let state_clone = state.clone();
    let start = Instant::now();
    std::thread::spawn(move || loop {
        let elapsed = start.elapsed();
        let new_string = format!("elapsed: {}", elapsed.as_secs());
        *state_clone.displayed_string.write(&Context::Empty) = new_string;
        let nanos_to_sleep = 5_000_000_000 - elapsed.as_nanos() % 1_000_000_000;
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
