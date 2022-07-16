//! Applin Demo
//! =================
//! Start the server with:
//! `cargo run --package applin --example demo`
//!
//! Then connect to it with an Applin client.
#![forbid(unsafe_code)]

use applin::action::{nothing, pop, push, rpc};
use applin::data::{Context, Roster};
use applin::session::{KeySet, PageKey, Session, SessionSet};
use applin::widget::{
    AlertModal, BackButton, Button, Column, DrawerModal, Empty, Form, FormButton, FormCheckbox,
    FormDetail, FormError, FormSection, ModalButton, NavPage, PlainPage, Text,
};
use core::fmt::Debug;
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{
    print_log_response, socket_addr_all_interfaces, ContentType, HttpServerBuilder, Request,
    Response, ResponseBody,
};
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

static RPC1_PATH: &str = "/rpc1";

fn rpc1(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    session.rpc_response()
}

fn add_alert_page(drawer: &PageKey, keys: &mut KeySet<SessionState>) -> PageKey {
    const KEY: &str = "/alert";
    let alert2 = keys.add_static_page("/alert2", AlertModal::new("Alert 2").with_ok());
    keys.add_static_page(
        KEY,
        AlertModal::new("Alert Modal").with_widgets((
            ModalButton::new("Alert 2").with_action(push(&alert2)),
            ModalButton::new("Alert Modal").with_action(push(&PageKey::new(KEY))),
            ModalButton::new("Drawer Modal").with_action(push(drawer)),
            ModalButton::new("Destructive Button")
                .with_is_destructive()
                .with_action(nothing()),
            ModalButton::cancel(),
            ModalButton::new(
                "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
            )
            .with_action(nothing()),
            ModalButton::new(
                "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
            )
            .with_action(nothing()),
            ModalButton::new("Disabled Button"),
        )),
    )
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
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                )
                .with_action(push(&pressed)),
                Button::new(
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
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
                FormButton::new("").with_action(push(&pressed)),
                FormButton::new(
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                )
                .with_action(push(&pressed)),
                FormButton::new(
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                )
                .with_action(push(&pressed)),
                FormButton::new("Disabled"),
                FormButton::new("Does Nothing").with_action(nothing()),
            )),
        ),
    )
}

fn add_form_checkbox_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-checkbox",
        NavPage::new(
            "Form Checkbox",
            Form::new((
                FormCheckbox::new("checkbox", "Checkbox"),
                FormCheckbox::new("initial-checked", "Initially checked").with_initial(true),
                FormCheckbox::new("with-rpc", "Does RPC on change")
                    .with_rpc("/form-checkbox-update"),
                FormCheckbox::new("checkbox", ""),
                FormCheckbox::new(
                    "checkbox",
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                ),
                FormCheckbox::new(
                    "checkbox",
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                ),
            )),
        ),
    )
}

#[allow(clippy::too_many_lines)]
fn add_form_detail_page(keys: &mut KeySet<SessionState>) -> PageKey {
    let pressed = keys.add_static_page(
        "/form-detail-pressed",
        NavPage::new("Form Detail Pressed", Empty::new()),
    );
    // NOTE: If rust-fmt refuses to format this, try making all lines shorter, under the limit.
    keys.add_static_page(
        "/form-detail",
        NavPage::new(
            "Form Detail",
            Form::new((
                FormSection::new().with_title("Text").with_widgets((
                    FormDetail::new("Text").with_action(push(&pressed)),
                    FormDetail::new("Disabled"),
                    FormDetail::new("Does Nothing").with_action(nothing()),
                    FormDetail::new("").with_action(push(&pressed)),
                    FormDetail::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_action(push(&pressed)),
                    FormDetail::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_action(push(&pressed)),
                )),
                FormSection::new()
                    .with_title("Text + Sub-text")
                    .with_widgets((
                    FormDetail::new("Text")
                        .with_sub_text("Sub-text")
                        .with_action(push(&pressed)),
                    FormDetail::new("Disabled").with_sub_text("Sub-text"),
                    FormDetail::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_action(push(&pressed)),
                    FormDetail::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_action(push(&pressed)),
                    FormDetail::new("Text")
                        .with_sub_text(
                            "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                        )
                        .with_action(push(&pressed)),
                    FormDetail::new("Text")
                        .with_sub_text(
                            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                        )
                        .with_action(push(&pressed)),
                    FormDetail::new("")
                        .with_sub_text("")
                        .with_action(push(&pressed)),
                    FormDetail::new("Sub-text is empty")
                        .with_sub_text("")
                        .with_action(push(&pressed)),
                    FormDetail::new("")
                        .with_sub_text("Text is empty")
                        .with_action(push(&pressed)),
                )),
                FormSection::new().with_title("Image + Text").with_widgets((
                    FormDetail::new("Text")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("Disabled").with_photo_url("/placeholder-200x200.png"),
                    FormDetail::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    FormDetail::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    FormDetail::new("")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("Image not found")
                        .with_photo_url("/nonexistent")
                        .with_action(push(&pressed)),
                    FormDetail::new("Not an image")
                        .with_photo_url("/health")
                        .with_action(push(&pressed)),
                    // TODO: Use a URL that never returns a result.
                )),
                FormSection::new()
                    .with_title("Image + Text + Sub-text")
                    .with_widgets((
                    FormDetail::new("Text")
                        .with_sub_text("Sub-text")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("Disabled")
                        .with_sub_text("Sub-text")
                        .with_photo_url("/placeholder-200x200.png"),
                    FormDetail::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    FormDetail::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    FormDetail::new("Text")
                        .with_sub_text(
                            "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                        )
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("Text")
                        .with_sub_text(
                            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                        )
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("")
                        .with_sub_text("")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("Sub-text is empty")
                        .with_sub_text("")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("")
                        .with_sub_text("Text is empty")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                )),
            )),
        ),
    )
}

fn add_form_error_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-error",
        NavPage::new(
            "Form Error",
            Form::new((
                FormError::new("Error Message"),
                FormError::new(""),
                FormError::new(
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                ),
                FormError::new(
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                ),
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
                    .with_widgets((Text::new("aaa"), Text::new("aaaa"))),
                FormSection::new().with_title("Empty Section"),
                FormSection::new()
                    .with_title("Section B")
                    .with_widgets((Text::new("bbb"), Text::new("bbbb"))),
                FormSection::new().with_widgets((
                    Text::new("First item of a section with no title."),
                    Text::new("Below is an empty section with no title."),
                )),
                FormSection::new(),
                FormSection::new()
                    .with_title("Section C")
                    .with_widgets((Text::new("ccc"), Text::new("cccc"))),
            )),
        ),
    )
}

fn add_form_text_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-text",
        NavPage::new(
            "Form Text",
            Form::new((
                Text::new("Text"),
                Text::new(""),
                Text::new("MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM"),
                Text::new("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"),
            )),
        ),
    )
}

fn add_nav_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/nav-page",
        NavPage::new(
            "Nav Page",
            Form::new((
                Text::new("Hello"), //
                Text::new("Hello 2"),
            )),
        ),
    )
}

fn add_plain_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/plain-page",
        PlainPage::new(
            "Plain Page",
            Form::new((
                Text::new("Hello"), //
                Button::new("Back").with_action(pop()),
            )),
        ),
    )
}

fn add_inert_page(state: &Arc<ServerState>, keys: &mut KeySet<SessionState>) -> PageKey {
    let state_clone = state.clone();
    keys.add_page_fn("/inert", move |rebuilder| {
        Ok(NavPage::new(
            "Inert",
            Column::new((
                Text::new(format!(
                    "epoch seconds: {}",
                    state_clone.clock_epoch_seconds.read(rebuilder)
                )),
                Text::new("The home page has poll=10, so you will see this page update when the app polls."),
            )),
        ))
    })
}

fn add_poll_page(state: &Arc<ServerState>, keys: &mut KeySet<SessionState>) -> PageKey {
    let state_clone = state.clone();
    keys.add_page_fn("/poll", move |rebuilder| {
        Ok(NavPage::new(
            "Poll Every 2 Seconds",
            Column::new((
                Text::new(format!(
                    "epoch seconds: {}",
                    state_clone.clock_epoch_seconds.read(rebuilder)
                )),
                // Checkbox::new("clock-check0"),
                // Text::new("Hello"),
            )),
        )
        .with_poll(2))
    })
}

fn add_stream_page(state: &Arc<ServerState>, keys: &mut KeySet<SessionState>) -> PageKey {
    let state_clone = state.clone();
    keys.add_page_fn("/stream", move |rebuilder| {
        Ok(NavPage::new(
            "Stream",
            Column::new((
                Text::new(format!(
                    "epoch seconds: {}",
                    state_clone.clock_epoch_seconds.read(rebuilder)
                )),
                // Checkbox::new("clock-check0"),
                // Text::new("Hello"),
            )),
        )
        .with_stream())
    })
}

fn key_set(state: &Arc<ServerState>) -> KeySet<SessionState> {
    let mut keys = KeySet::new();
    // Pages
    let drawer_modal = add_drawer_modal_page(&mut keys);
    let alert_modal = add_alert_page(&drawer_modal, &mut keys);
    let nav_page = add_nav_page(&mut keys);
    let plain_page = add_plain_page(&mut keys);
    // Widgets
    let back_buttons_page = add_back_button_pages(&mut keys);
    let buttons_page = add_button_page(&mut keys);
    let form_button_page = add_form_button_page(&mut keys);
    let form_checkbox_page = add_form_checkbox_page(&mut keys);
    let form_detail_page = add_form_detail_page(&mut keys);
    let form_error_page = add_form_error_page(&mut keys);
    let form_section_page = add_form_section_page(&mut keys);
    let form_text_page = add_form_text_page(&mut keys);
    // Update Modes
    let inert_page = add_inert_page(state, &mut keys);
    let poll_page = add_poll_page(state, &mut keys);
    let stream_page = add_stream_page(state, &mut keys);
    keys.add_static_page(
        "/",
        NavPage::new(
            "Applin Demo",
            Form::new((
                FormSection::new().with_title("Pages").with_widgets((
                    FormDetail::new("Alert Modal").with_action(push(&alert_modal)),
                    FormDetail::new("Drawer Modal").with_action(push(&drawer_modal)),
                    FormDetail::new("Nav Page").with_action(push(&nav_page)),
                    FormDetail::new("Plain Page").with_action(push(&plain_page)),
                )),
                FormSection::new().with_title("Widgets").with_widgets((
                    FormDetail::new("Back Button").with_action(push(&back_buttons_page)),
                    FormDetail::new("Button").with_action(push(&buttons_page)),
                    FormDetail::new("Form Button").with_action(push(&form_button_page)),
                    FormDetail::new("Form Checkbox").with_action(push(&form_checkbox_page)),
                    FormDetail::new("Form Detail").with_action(push(&form_detail_page)),
                    FormDetail::new("Form Error").with_action(push(&form_error_page)),
                    FormDetail::new("Form Section").with_action(push(&form_section_page)),
                    FormDetail::new("Form Text").with_action(push(&form_text_page)),
                )),
                FormSection::new().with_title("Update Modes").with_widgets((
                    FormDetail::new("Inert").with_action(push(&inert_page)),
                    FormDetail::new("Poll").with_action(push(&poll_page)),
                    FormDetail::new("Stream").with_action(push(&stream_page)),
                )),
            )),
        )
        .with_poll(10),
    );
    keys
}

fn get_or_new_session(
    state: &Arc<ServerState>,
    req: &Request,
) -> Result<Arc<Session<SessionState>>, Response> {
    let state_clone = state.clone();
    state.sessions.get_or_new(
        req,
        move |_rebuilder| Ok(key_set(&state_clone)),
        || SessionState {},
    )
}

fn handle_req(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    match (req.method(), req.url().path()) {
        ("GET", "/health") => Ok(Response::text(200, "ok")),
        ("GET", "/") => get_or_new_session(state, req)?.poll(),
        ("GET", "/stream") => get_or_new_session(state, req)?.stream(),
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
