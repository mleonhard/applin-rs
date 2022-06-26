//! Applin Demo
//! =================
//! Start the server with:
//! `cargo run --package applin --example demo`
//!
//! Then connect to it with an Applin client.
#![forbid(unsafe_code)]

use applin::builder::{
    nothing, pop, push, rpc, AlertModal, BackButton, Button, Column, DrawerModal, Empty, Form,
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

static RPC1_PATH: &'static str = "/rpc1";

fn rpc1(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    session.rpc_response()
}

fn add_back_button_pages(keys: &mut KeySet<SessionState>) -> PageKey {
    let default = keys.add_static_page(
        "/back-button-default",
        NavPage::new("Default", Empty::new()),
    );
    let disabled = keys.add_static_page(
        "/back-button-disabled",
        NavPage::new(
            "Disabled",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(BackButton::new()),
    );
    let missing = keys.add_static_page(
        "/back-button-missing",
        NavPage::new(
            "Missing",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .without_back(),
    );
    let rpc_ok = keys.add_static_page(
        "/back-button-rpc-ok",
        NavPage::new(
            "RPC",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(
            BackButton::new()
                .with_action(rpc(RPC1_PATH))
                .with_action(pop()),
        ),
    );
    let rpc_err = keys.add_static_page(
        "/back-button-rpc-error",
        NavPage::new(
            "RPC Error",
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
                FormDetail::new("Default").with_action(push(&default)),
                FormDetail::new("Disabled").with_action(push(&disabled)),
                FormDetail::new("Missing").with_action(push(&missing)),
                FormDetail::new("RPC").with_action(push(&rpc_ok)),
                FormDetail::new("RPC Error").with_action(push(&rpc_err)),
            )),
        ),
    )
}

fn add_clock_page(state: &Arc<ServerState>, keys: &mut KeySet<SessionState>) -> PageKey {
    let state_clone = state.clone();
    keys.add_page_fn("/clock", move |ctx| {
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
    })
}

fn add_drawer_modal_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/drawer-modal",
        DrawerModal::new("Drawer1").with_widgets((
            ModalButton::cancel(),
            ModalButton::new("Save")
                .with_action(rpc("/example-method"))
                .with_action(pop()),
        )),
    )
}

fn add_alert_page(drawer: &PageKey, keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/alert",
        AlertModal::new("Title1").with_widgets((
            ModalButton::new("Drawer Modal").with_action(push(&drawer)),
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
    )
}

fn add_button_page(keys: &mut KeySet<SessionState>) -> PageKey {
    let pressed = keys.add_static_page(
        "/button-pressed",
        AlertModal::new("Button Pressed").with_ok(),
    );
    keys.add_static_page(
        "/button",
        NavPage::new(
            "Button",
            Form::new((
                Button::new("Button").with_action(push(&pressed)),
                Button::new(
                    "Very Very Very Very Very Very Very Very Very Very Very Very Very Long Label",
                )
                .with_action(push(&pressed)),
                Button::new(
                    "Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm",
                )
                .with_action(push(&pressed)),
                Text::new("Button with empty label:"),
                Button::new("").with_action(push(&pressed)),
                Button::new("Disabled Button"),
                Button::new("Does Nothing").with_action(nothing()),
            )),
        ),
    )
}

fn add_form_detail_page(keys: &mut KeySet<SessionState>) -> PageKey {
    let pressed = keys.add_static_page(
        "/form-detail-pressed",
        NavPage::new("Form Detail Pressed", Empty::new()),
    );
    // TODO: File bug about rust-fmt's failure to format this function call.
    keys.add_static_page(
        "/form-detail",
        NavPage::new(
            "Form Detail",
            Form::new((
                FormDetail::new("Form Detail").with_action(push(&pressed)),
                FormDetail::new("Form Detail")
                    .with_sub_text("with sub-text")
                    .with_action(push(&pressed)),
                FormDetail::new("Form Detail with Photo")
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                FormDetail::new("Form Detail with Photo")
                    .with_sub_text("with sub-text")
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                FormDetail::new("Form Detail With Very Very Very Very Very Very Very Very Very Very Very Long Text")
                    .with_action(push(&pressed)),
                FormDetail::new("Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm")
                    .with_action(push(&pressed)),
                Text::new("Form Detail with empty label:"),
                FormDetail::new("").with_action(push(&pressed)),
                FormDetail::new("Disabled Form Detail"),
                FormDetail::new("Does Nothing").with_action(nothing()),
            )),
        ),
    )
}

fn add_form_text_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-text",
        NavPage::new(
            "Form Detail",
            Form::new((
                Text::new("text1"),
                Text::new("Empty text:"),
                Text::new(""),
                Text::new("Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Long Text"),
                Text::new("Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm"),
            )),
        ),
    )
}

fn add_form_button_page(keys: &mut KeySet<SessionState>) -> PageKey {
    let pressed = keys.add_static_page(
        "/form-button-pressed",
        AlertModal::new("Form Button Pressed").with_ok(),
    );
    keys.add_static_page(
        "/form-button",
        NavPage::new(
            "Form Button",
            Form::new((
                FormButton::new("Button1").with_action(push(&pressed)),
                Text::new("Button with empty label:"),
                FormButton::new("").with_action(push(&pressed)),
                FormButton::new("Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Very Long Text")
                    .with_action(push(&pressed)),
                FormButton::new("Mmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmm")
                    .with_action(push(&pressed)),
                FormButton::new("Disabled"),
                FormButton::new("Does Nothing").with_action(nothing()),
            )),
        ),
    )
}

fn add_form_section_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-section",
        NavPage::new(
            "Form Section",
            Form::new((
                FormSection::new()
                    .with_title("Section A")
                    .with_widgets((Text::new("one"),)),
                FormSection::new()
                    .with_title("Section B")
                    .with_widgets((Text::new("one"), Text::new("two"))),
                FormSection::new()
                    .with_widgets((Text::new("This section has no title."), Text::new("two"))),
                FormSection::new().with_title("Empty Section"),
                FormSection::new(),
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
    let clock_page = add_clock_page(state, &mut keys);
    let drawer_modal = add_drawer_modal_page(&mut keys);
    let alert_modal = add_alert_page(&drawer_modal, &mut keys);
    let back_buttons_page = add_back_button_pages(&mut keys);
    let buttons_page = add_button_page(&mut keys);
    let form_detail_page = add_form_detail_page(&mut keys);
    let form_text_page = add_form_text_page(&mut keys);
    let form_button_page = add_form_button_page(&mut keys);
    let form_section_page = add_form_section_page(&mut keys);
    keys.add_static_page(
        "/",
        PlainPage::new(
            "Applin Demo",
            Form::new((
                Button::new("Clock Page").with_action(push(&clock_page)),
                FormDetail::new("Alert Modal").with_action(push(&alert_modal)),
                FormDetail::new("Drawer Modal").with_action(push(&drawer_modal)),
                FormDetail::new("Back Button").with_action(push(&back_buttons_page)),
                FormDetail::new("Button").with_action(push(&buttons_page)),
                FormDetail::new("Form Detail").with_action(push(&form_detail_page)),
                FormDetail::new("Form Text").with_action(push(&form_text_page)),
                FormDetail::new("Form Button").with_action(push(&form_button_page)),
                FormDetail::new("Form Section").with_action(push(&form_section_page)),
            )),
        ),
    );
    Ok(keys)
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
        ("POST", path) if path == RPC1_PATH => rpc1(state, req),
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
