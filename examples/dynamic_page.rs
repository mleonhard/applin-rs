//! Dynamic Page Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package maggie --example dynamic_page
//! Access the app at http://127.0.0.1:8000/
//! INFO GET / => 200 streamed
//! ```
//!
//! Connect to it and see the updates arrive periodically:
//! ```
//! $ curl http://127.0.0.1:8000/
//! data: {"pages":{"/":{"title":"Dynamic Page Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"The page below appears and disappears every 5 seconds:","typ":"text"},{"typ":"empty"}]}}}}
//! data: {"pages":{"/page_2":{"title":"Page 2","typ":"nav-page","widget":{"text":"This is page 2.","typ":"text"}}}}
//! data: {"pages":{"/":{"title":"Dynamic Page Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"The page below appears and disappears every 5 seconds:","typ":"text"},{"actions":["/page_2"],"text":"Page 2","typ":"detail-cell"}]}}}}
//! data: {"pages":{"/page_2":null}}
//! data: {"pages":{"/":{"title":"Dynamic Page Example","typ":"nav-page","widget":{"typ":"column","widgets":[{"text":"The page below appears and disappears every 5 seconds:","typ":"text"},{"typ":"empty"}]}}}}
//! ^C
//! ```
#![forbid(unsafe_code)]

use beatrice::reexport::permit::Permit;
use beatrice::reexport::{safina_executor, safina_timer};
use beatrice::{print_log_response, socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
use maggie::builder::{empty, push, Column, DetailCell, NavPage, Text};
use maggie::data::{Context, Roster};
use maggie::page::KeySet;
use maggie::session::SessionSet;
use std::error::Error;
use std::ops::BitXorAssign;
use std::sync::Arc;
use std::time::Duration;

struct SessionState {}

struct ServerState {
    show_page_2: Roster<bool, SessionState>,
    sessions: SessionSet<SessionState>,
}
impl ServerState {
    pub fn new(executor: &Arc<safina_executor::Executor>) -> Self {
        Self {
            show_page_2: Roster::new(false).with_cleanup_task(executor),
            sessions: SessionSet::new(executor),
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
fn key_set(
    state: &Arc<ServerState>,
    ctx: &Context<SessionState>,
) -> Result<KeySet<SessionState>, Box<dyn Error>> {
    let mut keys = KeySet::new();
    let state_clone = state.clone();
    keys.add_page_fn("/", move |ctx| {
        Ok(NavPage::new(
            "Dynamic Page Example",
            Column::new((
                Text::new("The page below appears and disappears every 5 seconds:"),
                if *state_clone.show_page_2.read(ctx) {
                    DetailCell::new("Page 2")
                        .with_action(push("/page_2"))
                        .into()
                } else {
                    empty()
                },
            )),
        ))
    });
    if *state.show_page_2.read(ctx) {
        keys.add_static_page(
            "/page_2",
            NavPage::new("Page 2", Text::new("This is page 2.")),
        );
    }
    Ok(keys)
}

#[allow(clippy::unnecessary_wraps)]
fn connect(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let state_clone = state.clone();
    let (_session, response) = state.sessions.resume_or_new(
        req,
        move |ctx| key_set(&state_clone, ctx),
        || SessionState {},
    )?;
    Ok(response)
}

#[allow(clippy::unnecessary_wraps)]
fn handle_req(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    match (req.method(), req.url().path()) {
        ("GET", "/health") => Ok(Response::text(200, "ok")),
        ("GET", "/") => connect(state, req),
        _ => Ok(Response::text(404, "Not found")),
    }
}

pub fn main() {
    println!("Access the app at http://127.0.0.1:8000/");
    safina_timer::start_timer_thread();
    let executor = safina_executor::Executor::default();
    let state = Arc::new(ServerState::new(&executor));
    let permit = Permit::new();
    let state_clone = state.clone();
    let toggler_thread_permit = permit.new_sub();
    std::thread::spawn(move || {
        while !toggler_thread_permit.is_revoked() {
            std::thread::sleep(Duration::from_secs(5));
            state_clone
                .show_page_2
                .write(&Context::Empty)
                .bitxor_assign(true);
        }
    });
    let request_handler = move |req: Request| print_log_response(&req, handle_req(&state, &req));
    executor
        .block_on(
            HttpServerBuilder::new()
                .listen_addr(socket_addr_127_0_0_1(8000))
                .spawn_and_join(request_handler),
        )
        .unwrap();
}