//! Dynamic Page Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package maggie --example dynamic_page
//! Access the app at http://127.0.0.1:8000/maggie
//! INFO GET /connect => 200 streamed
//! ```
//!
//! Connect to it and see the updates arrive periodically:
//! ```
//! $ curl http://127.0.0.1:8000/connect
//! data: {"/":[{"text":"Dynamic Page Example","typ":"title-bar"},"The page below appears and disappears every 5 seconds:",null]}
//! data: {"/page_2":[{"start_actions":["pop"],"start_text":"Back","text":"Page 2","typ":"title-bar"},"This is page 2."]}
//! data: {"/":[{"text":"Dynamic Page Example","typ":"title-bar"},"The page below appears and disappears every 5 seconds:",{"actions":["/page_2"],"text":"Page 2","typ":"detail-cell"}]}
//! data: {"/page_2":null}
//! data: {"/":[{"text":"Dynamic Page Example","typ":"title-bar"},"The page below appears and disappears every 5 seconds:",null]}
//! data: {"/page_2":[{"start_actions":["pop"],"start_text":"Back","text":"Page 2","typ":"title-bar"},"This is page 2."]}
//! data: {"/":[{"text":"Dynamic Page Example","typ":"title-bar"},"The page below appears and disappears every 5 seconds:",{"actions":["/page_2"],"text":"Page 2","typ":"detail-cell"}]}
//! data: {"/page_2":null}
//! data: {"/":[{"text":"Dynamic Page Example","typ":"title-bar"},"The page below appears and disappears every 5 seconds:",null]}
//! ^C
//! ```
#![forbid(unsafe_code)]

use crate::safina_executor::Executor;
use beatrice::reexport::permit::Permit;
use beatrice::reexport::{safina_executor, safina_timer};
use beatrice::{print_log_response, socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
use maggie::context::Context;
use maggie::key_set::KeySet;
use maggie::roster::Roster;
use maggie::session_set::SessionSet;
use maggie::widgets::{text, DetailCell, TitleBar};
use serde_json::Value;
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
    pub fn new(executor: &Arc<Executor>) -> Self {
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
    keys.add_fn("/", move |ctx| {
        Ok(Value::Array(vec![
            TitleBar::new("Dynamic Page Example").build(),
            text("The page below appears and disappears every 5 seconds:"),
            if *state_clone.show_page_2.read(ctx) {
                DetailCell::new("Page 2").with_action("/page_2").build()
            } else {
                Value::Null
            },
        ]))
    });
    if *state.show_page_2.read(ctx) {
        keys.add_static(
            "/page_2",
            Value::Array(vec![
                TitleBar::new("Page 2").with_back().build(),
                text("This is page 2."),
            ]),
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
        ("GET", "/connect") => connect(state, req),
        _ => Ok(Response::text(404, "Not found")),
    }
}

pub fn main() {
    println!("Access the app at http://127.0.0.1:8000/maggie");
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
    let request_handler = move |req: Request| {
        print_log_response(
            req.method().to_string(),
            req.url().clone(),
            handle_req(&state, &req),
        )
    };
    executor
        .block_on(
            HttpServerBuilder::new()
                .listen_addr(socket_addr_127_0_0_1(8000))
                .spawn_and_join(request_handler),
        )
        .unwrap();
}
