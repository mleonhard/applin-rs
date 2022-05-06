//! Maggie Demo
//! =================
//! Start the server with:
//! `cargo run --package maggie --example demo`
//!
//! Then connect to it with a Maggie client.
#![forbid(unsafe_code)]

use beatrice::reexport::{safina_executor, safina_timer};
use beatrice::{
    print_log_response, socket_addr_all_interfaces, HttpServerBuilder, Request, Response,
};
use maggie::builder::{
    empty, ok_button, pop, push, rpc, BackButton, Button, Column, DetailCell, InfoModal, List,
    NavPage, Text,
};
use maggie::data::Context;
use maggie::page::KeySet;
use maggie::session::SessionSet;
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
struct SessionState {}

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
    keys.add_static_page(
        "/",
        NavPage::new(
            "Maggie Demo",
            List::new()
                .with_widgets((DetailCell::new("Back Button").with_action(push("/back-button")),)),
        ),
    );
    keys.add_static_page(
        "/back-button",
        NavPage::new(
            "Back Button",
            List::new().with_widgets((
                Text::new("↑ the default back button"),
                DetailCell::new("Disabled Back Button").with_action(push("/back-button-disabled")),
                DetailCell::new("Missing Back Button").with_action(push("/back-button-missing")),
                DetailCell::new("RPC Back Button").with_action(push("/back-button-rpc-ok")),
                DetailCell::new("RPC Error Back Button")
                    .with_action(push("/back-button-rpc-error")),
            )),
        ),
    );
    keys.add_static_page(
        "/back-button-disabled",
        NavPage::new(
            "Disabled Back Button",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(BackButton::new()),
    );
    keys.add_static_page(
        "/back-button-missing",
        NavPage::new(
            "Missing Back Button",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(empty()),
    );
    keys.add_static_page(
        "/rpc-succeeded-modal",
        InfoModal::new("RPC Succeeded").with_widget(ok_button()),
    );
    keys.add_static_page(
        "/back-button-rpc-ok",
        NavPage::new(
            "RPC Back Button",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(
            BackButton::new()
                .with_action(rpc("/example-method"))
                .with_action(pop()),
        ),
    );
    keys.add_static_page(
        "/back-button-rpc-error",
        NavPage::new(
            "RPC Error Back Button",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(
            BackButton::new()
                .with_action(rpc("/nonexistent-method"))
                .with_action(pop()),
        ),
    );
    Ok(keys)
}

fn example_method(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    session.rpc_response()
}

fn connect(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let state_clone = state.clone();
    let (_session, response) = state.sessions.resume_or_new(
        req,
        move |ctx| key_set(&state_clone, ctx),
        || SessionState {},
    )?;
    Ok(response)
}

fn handle_req(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    match (req.method(), req.url().path()) {
        ("GET", "/health") => Ok(Response::text(200, "ok")),
        ("GET", "/") => connect(state, req),
        ("POST", "/example-method") => example_method(state, req),
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
                .listen_addr(socket_addr_all_interfaces(8000))
                .max_conns(100)
                .spawn_and_join(request_handler),
        )
        .unwrap();
}
