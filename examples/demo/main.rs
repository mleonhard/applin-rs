//! Applin Demo
//! =================
//! Start the server with:
//! `cargo run --package applin --example demo`
//!
//! Then connect to it with an Applin client.
#![forbid(unsafe_code)]

mod form_widgets;
mod pages;
mod updates;
mod vars;
mod widgets;

use applin::action::push;
use applin::data::Roster;
use applin::session::{KeySet, Session, SessionSet};
use applin::widget::{Form, FormDetail, FormSection, NavPage};
use core::fmt::Debug;
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{
    print_log_response, socket_addr_all_interfaces, ContentType, HttpServerBuilder, Request,
    Response, ResponseBody,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct SessionState {}

pub struct ServerState {
    clock_epoch_seconds: Roster<u64, SessionState>,
    sessions: SessionSet<SessionState>,
}
impl ServerState {
    #[must_use]
    pub fn new(executor: &Arc<safina_executor::Executor>) -> Self {
        Self {
            clock_epoch_seconds: Roster::new(0),
            sessions: SessionSet::new(executor),
        }
    }
}

fn key_set(state: &Arc<ServerState>) -> KeySet<SessionState> {
    let mut keys = KeySet::new();
    // Pages
    let drawer_modal = pages::add_drawer_modal_page(&mut keys);
    let alert_modal = pages::add_alert_page(&drawer_modal, &mut keys);
    let nav_page = pages::add_nav_page(&mut keys);
    let plain_page = pages::add_plain_page(&mut keys);
    // Widgets
    let back_buttons_page = widgets::add_back_button_pages(&mut keys);
    let buttons_page = widgets::add_button_page(&mut keys);
    let form_button_page = form_widgets::add_form_button_page(&mut keys);
    let form_checkbox_page = form_widgets::add_form_checkbox_page(&mut keys);
    let form_detail_page = form_widgets::add_form_detail_page(&mut keys);
    let form_error_page = form_widgets::add_form_error_page(&mut keys);
    let form_section_page = form_widgets::add_form_section_page(&mut keys);
    let form_text_page = form_widgets::add_form_text_page(&mut keys);
    // Update Modes
    let inert_page = updates::add_inert_page(state, &mut keys);
    let poll_page = updates::add_poll_page(state, &mut keys);
    let stream_page = updates::add_stream_page(state, &mut keys);
    // Vars
    let check_vars_page = vars::add_check_vars_page(&mut keys);
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
                FormSection::new().with_title("Vars").with_widgets((
                    FormDetail::new("Check Vars").with_action(push(&check_vars_page)),
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
        ("POST", path) if path == pages::SAVE_RPC_PATH => pages::save_rpc(state, req),
        ("POST", path) if path == widgets::BACK_RPC_PATH => widgets::back_rpc(state, req),
        ("POST", path) if path == form_widgets::FORM_CHECKBOX_RPC_PATH => {
            form_widgets::form_checkbox_rpc(state, req)
        }
        ("POST", path) if path == vars::CHECK_VARS_RPC_PATH => vars::check_vars_rpc(state, req),
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
    updates::start_updater_thread(state.clone());
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
