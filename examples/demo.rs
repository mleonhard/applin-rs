//! Applin Demo
//! =================
//! Start the server with:
//! `cargo run --package applin --example demo`
//!
//! Then connect to it with an Applin client.
#![forbid(unsafe_code)]

use applin::builder::{
    empty, nothing, pop, push, rpc, BackButton, Button, Column, DetailCell, List, NavPage, Text,
};
use applin::data::Context;
use applin::page::{KeySet, PageKey};
use applin::session::SessionSet;
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{
    print_log_response, socket_addr_all_interfaces, ContentType, HttpServerBuilder, Request,
    Response, ResponseBody,
};
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

fn add_back_button_pages(keys: &mut KeySet<SessionState>) -> PageKey {
    let back_button_default = keys.add_static_page(
        "/back-button-default",
        NavPage::new("Default Back Button", empty()),
    );
    let back_button_disabled = keys.add_static_page(
        "/back-button-disabled",
        NavPage::new(
            "Disabled Back Button",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(BackButton::new()),
    );
    let back_button_missing = keys.add_static_page(
        "/back-button-missing",
        NavPage::new(
            "Missing Back Button",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(empty()),
    );
    let back_button_rpc_ok = keys.add_static_page(
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
    let back_button_rpc_error = keys.add_static_page(
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
    keys.add_static_page(
        "/back-button",
        NavPage::new(
            "Back Button",
            List::new().with_widgets((
                DetailCell::new("Default Back Button").with_action(push(back_button_default)),
                DetailCell::new("Disabled Back Button").with_action(push(back_button_disabled)),
                DetailCell::new("Missing Back Button").with_action(push(back_button_missing)),
                DetailCell::new("RPC Back Button").with_action(push(back_button_rpc_ok)),
                DetailCell::new("RPC Error Back Button").with_action(push(back_button_rpc_error)),
            )),
        ),
    )
}

#[allow(clippy::unnecessary_wraps)]
fn key_set(
    _state: &Arc<ServerState>,
    _ctx: &Context<SessionState>,
) -> Result<KeySet<SessionState>, Box<dyn Error>> {
    let mut keys = KeySet::new();
    let back_buttons_page = add_back_button_pages(&mut keys);
    let buttons_page = keys.add_static_page(
        "/button",
        NavPage::new(
            "Button",
            Column::new((
                Button::new("Button").with_action(nothing()),
                Button::new(
                    "Button With Very Very Very Very Very Very Very Very Very Very Very Long Text",
                )
                    .with_action(nothing()),
                Button::new(
                    "Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm",
                )
                    .with_action(nothing()),
                Text::new("Button with empty label:"),
                Button::new("").with_action(nothing()),
                Button::new("Disabled Button"),
                List::new().with_title("A List").with_widgets((
                    Button::new("Button").with_action(nothing()),
                    Button::new(
                        "Button With Very Very Very Very Very Very Very Very Very Very Very Long Text",
                    )
                        .with_action(nothing()),
                    Button::new(
                        "Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm",
                    )
                        .with_action(nothing()),
                    Text::new("Button with empty label:"),
                    Button::new("").with_action(nothing()),
                    Button::new("Disabled Button"),
                )),
            )),
        ),
    );
    let detail_cell_page = keys.add_static_page(
        "/detail-cell",
        NavPage::new(
            "Detail Cell",
            Column::new((
                DetailCell::new("Detail Cell").with_action(nothing()),
                DetailCell::new("Detail Cell with Photo").with_photo_url("/placeholder-200x200.png").with_action(nothing()),
                DetailCell::new("Detail Cell With Very Very Very Very Very Very Very Very Very Very Very Long Text")
                    .with_action(nothing()),
                DetailCell::new("Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm")
                    .with_action(nothing()),
                Text::new("Detail Cell with empty label:"),
                DetailCell::new("").with_action(nothing()),
                DetailCell::new("Disabled Detail Cell"),
                List::new().with_title("A List").with_widgets((
                    DetailCell::new("Detail Cell").with_action(nothing()),
                    DetailCell::new("Detail Cell with Photo").with_photo_url("/placeholder-200x200.png").with_action(nothing()),
                    DetailCell::new("Detail Cell With Very Very Very Very Very Very Very Very Very Very Very Long Text")
                        .with_action(nothing()),
                    DetailCell::new("Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm")
                        .with_action(nothing()),
                    Text::new("Detail Cell with empty label:"),
                    DetailCell::new("").with_action(nothing()),
                    DetailCell::new("Disabled Detail Cell"),
                )),
            )),
        ),
    );
    keys.add_static_page(
        "/",
        NavPage::new(
            "Applin Demo",
            List::new().with_widgets((
                DetailCell::new("Back Button").with_action(push(back_buttons_page)),
                DetailCell::new("Button").with_action(push(buttons_page)),
                DetailCell::new("Detail Cell").with_action(push(detail_cell_page)),
            )),
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
        ("GET", "/placeholder-200x200.png") => Ok(Response::new(200)
            .with_type(ContentType::Png)
            .with_body(ResponseBody::StaticBytes(include_bytes!(
                "placeholder-200x200.png"
            )))),
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
                .listen_addr(socket_addr_all_interfaces(8000))
                .max_conns(100)
                .spawn_and_join(request_handler),
        )
        .unwrap();
}
