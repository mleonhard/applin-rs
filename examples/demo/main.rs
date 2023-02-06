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
use applin::session::{ApplinSession, PageMap, SessionSet};
use applin::widget::{Column, FormSection, NavButton, NavPage, Scroll};
use core::fmt::Debug;
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{
    print_log_response, socket_addr_all_interfaces, ContentType, HttpServerBuilder, Request,
    Response, ResponseBody,
};
use std::sync::Arc;

pub const CHECK_VARS_RPC_PATH: &str = "/check-vars-rpc";
pub const ERROR_RPC_PATH: &str = "/error";
pub const OK_RPC_PATH: &str = "/ok";

#[derive(Debug)]
pub struct Session {}

pub struct ServerState {
    clock_epoch_seconds: Roster<u64, Session>,
    sessions: SessionSet<Session>,
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

fn page_map(state: &Arc<ServerState>) -> PageMap<Session> {
    let mut keys = PageMap::new();
    // Pages
    let drawer_modal = pages::add_drawer_modal_page(&mut keys);
    let alert_modal = pages::add_alert_page(&drawer_modal, &mut keys);
    let nav_page = pages::add_nav_page(&mut keys);
    let plain_page = pages::add_plain_page(&mut keys);
    // Widgets
    let back_buttons_page = widgets::add_back_button_pages(&mut keys);
    let buttons_page = widgets::add_button_page(&mut keys);
    let checkbox_page = widgets::add_checkbox_page(&mut keys);
    let error_text_page = widgets::add_error_text_page(&mut keys);
    let nav_button_page = widgets::add_nav_button_page(&mut keys);
    let form_button_page = form_widgets::add_form_button_page(&mut keys);
    let form_section_page = form_widgets::add_form_section_page(&mut keys);
    let textfield_page = widgets::add_textfield_page(&mut keys);
    let image_page = widgets::add_image_page(&mut keys);
    let text_page = widgets::add_text_page(&mut keys);
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
            Scroll::new(Column::new((
                FormSection::new().with_title("Pages").with_widgets((
                    NavButton::new("Alert Modal").with_action(push(&alert_modal)),
                    NavButton::new("Drawer Modal").with_action(push(&drawer_modal)),
                    NavButton::new("Nav Page").with_action(push(&nav_page)),
                    NavButton::new("Plain Page").with_action(push(&plain_page)),
                )),
                FormSection::new().with_title("Widgets").with_widgets((
                    NavButton::new("Back Button").with_action(push(&back_buttons_page)),
                    NavButton::new("Button").with_action(push(&buttons_page)),
                    NavButton::new("Checkbox").with_action(push(&checkbox_page)),
                    NavButton::new("Error Text").with_action(push(&error_text_page)),
                    NavButton::new("Nav Button").with_action(push(&nav_button_page)),
                    NavButton::new("Form Button").with_action(push(&form_button_page)),
                    NavButton::new("Form Section").with_action(push(&form_section_page)),
                    NavButton::new("Image").with_action(push(&image_page)),
                    NavButton::new("Textfield").with_action(push(&textfield_page)),
                    NavButton::new("Text").with_action(push(&text_page)),
                )),
                FormSection::new().with_title("Update Modes").with_widgets((
                    NavButton::new("Inert").with_action(push(&inert_page)),
                    NavButton::new("Poll").with_action(push(&poll_page)),
                    NavButton::new("Stream").with_action(push(&stream_page)),
                )),
                FormSection::new().with_title("Vars").with_widgets((
                    NavButton::new("Check Vars").with_action(push(&check_vars_page)),
                )),
            ))),
        )
        .with_poll(10),
    );
    keys
}

fn get_or_new_session(
    state: &Arc<ServerState>,
    req: &Request,
) -> Result<Arc<ApplinSession<Session>>, Response> {
    let state_clone = state.clone();
    state.sessions.get_or_new(
        req,
        move |_rebuilder| Ok(page_map(&state_clone)),
        || Session {},
    )
}

#[allow(clippy::missing_errors_doc)]
pub fn ok_rpc(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    session.rpc_response()
}

fn handle_req(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    match (req.method(), req.url().path()) {
        ("GET", "/health") => Ok(Response::text(200, "ok")),
        ("POST", "/") => get_or_new_session(state, req)?.rpc_response(),
        ("GET", "/") => get_or_new_session(state, req)?.poll(),
        ("GET", "/stream") => get_or_new_session(state, req)?.stream(),
        ("POST", ERROR_RPC_PATH) => Err(Response::text(500, "error1")),
        ("POST", OK_RPC_PATH) => ok_rpc(state, req),
        ("POST", CHECK_VARS_RPC_PATH) => vars::check_vars_rpc(state, req),
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
