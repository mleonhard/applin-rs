//! Applin Demo
//! =================
//! Start the server with:
//! `cargo run --package applin --example demo`
//!
//! Then connect to it with an Applin client.
#![forbid(unsafe_code)]

use applin::builder::{
    empty, nothing, pop, push, rpc, AlertModal, BackButton, Button, Column, DrawerModal, Form,
    FormButton, FormDetail, FormSection, ModalButton, NavPage, PlainPage, Text,
};
use applin::data::{Context, Roster};
use applin::page::{KeySet, PageKey};
use applin::session::SessionSet;
use core::fmt::Debug;
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{
    print_log_response, socket_addr_all_interfaces, ContentType, HttpServerBuilder, Request,
    Response, ResponseBody,
};
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

fn epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Debug)]
struct SessionState {}

struct ServerState {
    clock_epoch_seconds: Roster<u64, SessionState>,
    sessions: SessionSet<SessionState>,
}
impl ServerState {
    pub fn new(executor: &Arc<safina_executor::Executor>) -> Self {
        Self {
            clock_epoch_seconds: Roster::new(epoch_seconds()),
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
        .without_back(),
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
            Form::new((
                FormDetail::new("Default Back Button").with_action(push(back_button_default)),
                FormDetail::new("Disabled Back Button").with_action(push(back_button_disabled)),
                FormDetail::new("Missing Back Button").with_action(push(back_button_missing)),
                FormDetail::new("RPC Back Button").with_action(push(back_button_rpc_ok)),
                FormDetail::new("RPC Error Back Button").with_action(push(back_button_rpc_error)),
            )),
        ),
    )
}

#[allow(clippy::unnecessary_wraps)]
fn key_set(
    state: &Arc<ServerState>,
    _ctx: &Context<SessionState>,
) -> Result<KeySet<SessionState>, Box<dyn Error>> {
    let mut keys = KeySet::new();
    let state_clone = state.clone();
    let clock_page = keys.add_page_fn("/clock", move |ctx| {
        Ok(NavPage::new(
            "Clock",
            Column::new((
                Text::new(format!(
                    "epoch seconds: {}",
                    state_clone.clock_epoch_seconds.read(ctx)
                )),
                // Checkbox::new("clock-check0"),
                // Text::new("Hello"),
            )),
        ))
    });
    let drawer_modal = keys.add_static_page(
        "/drawer-modal",
        DrawerModal::new("Drawer1").with_widgets((
            ModalButton::cancel(),
            ModalButton::new("Save")
                .with_action(rpc("/example-method"))
                .with_action(pop()),
        )),
    );
    let alert_modal = keys.add_static_page(
        "/alert",
        AlertModal::new("Title1").with_widgets((
            ModalButton::new("Drawer Modal").with_action(push(drawer_modal.clone())),
            ModalButton::new("Destructive Button")
                .with_is_destructive()
                .with_action(nothing()),
            ModalButton::cancel(),
            ModalButton::new(
                "Button With Very Very Very Very Very Very Very Very Very Very Very Long Text",
            )
            .with_action(nothing()),
            ModalButton::new(
                "Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm",
            )
            .with_action(nothing()),
            ModalButton::new("Disabled Button"),
        )),
    );
    let back_buttons_page = add_back_button_pages(&mut keys);
    let buttons_page = keys.add_static_page(
        "/button",
        NavPage::new(
            "Button",
            Form::new((
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
        ),
    );
    // TODO: File bug about rust-fmt's failure to format this function call.
    let form_page = keys.add_static_page(
        "/form",
        NavPage::new(
            "Form",
            Form::new((
                FormSection::new().with_title("Form Detail Widgets").with_widgets((
                    FormDetail::new("Form Detail").with_action(nothing()),
                    FormDetail::new("Form Detail").with_sub_text("with sub-text").with_action(nothing()),
                    FormDetail::new("Form Detail with Photo").with_photo_url("/placeholder-200x200.png").with_action(nothing()),
                    FormDetail::new("Form Detail with Photo").with_sub_text("with sub-text").with_photo_url("/placeholder-200x200.png").with_action(nothing()),
                    FormDetail::new("Form Detail With Very Very Very Very Very Very Very Very Very Very Very Long Text")
                        .with_action(nothing()),
                    FormDetail::new("Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm")
                        .with_action(nothing()),
                    Text::new("Form Detail with empty label:"),
                    FormDetail::new("").with_action(nothing()),
                    FormDetail::new("Disabled Form Detail"),
                )),
                FormSection::new().with_title("Text Widgets").with_widgets((
                    Text::new("text1"),
                    Text::new("Empty text:"),
                    Text::new(""),
                    Text::new("Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Long Text"),
                    Text::new("Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm"),
                )),
                FormSection::new().with_title("Form Button Widgets").with_widgets((
                    FormButton::new("Button1"),
                    Text::new("Button with empty label:"),
                    FormButton::new(""),
                    FormButton::new("Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Long Text"),
                    FormButton::new("Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm"),
                )),
            )),
        ),
    );
    keys.add_static_page(
        "/",
        PlainPage::new(
            "Applin Demo",
            Form::new((
                Button::new("Clock Page").with_action(push(clock_page)),
                FormDetail::new("Alert Modal").with_action(push(alert_modal)),
                FormDetail::new("Drawer Modal").with_action(push(drawer_modal)),
                FormDetail::new("Back Button").with_action(push(back_buttons_page)),
                FormDetail::new("Button").with_action(push(buttons_page)),
                FormDetail::new("Form Detail").with_action(push(form_page)),
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
            .with_max_age_seconds(365 * 24 * 60 * 60)
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
    let state_clone = state.clone();
    std::thread::spawn(move || loop {
        *state_clone.clock_epoch_seconds.write(&Context::Empty) = epoch_seconds();
        std::thread::sleep(Duration::from_secs(1));
    });
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
