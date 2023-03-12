//! Dynamic Page Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package applin --example dynamic_page
//! Access the app at http://127.0.0.1:8000/
//! INFO GET /stream => 200 streamed
//! ```
//!
//! Connect to it and see the updates arrive periodically:
//! ```
//! $ curl --include http://127.0.0.1:8000/stream
//! HTTP/1.1 200 OK
//! content-type: text/event-stream
//! transfer-encoding: chunked
//! set-cookie: session=5753770736856528337-7154909175280483379; HttpOnly; Max-Age=2592000; SameSite=Strict
//! cache-control: no-store
//!
//! data: {"pages":{"/":{"poll-seconds":5,"title":"Dynamic Page Example","typ":"nav-page","widget":{"h-alignment":"start","typ":"column","widgets":[{"text":"The page below appears and disappears every 5 seconds:","typ":"text"},{"actions":["push:/page_2"],"text":"Page 2","typ":"button"}]}},"/page_2":{"title":"Page 2","typ":"nav-page","widget":{"text":"This is page 2.","typ":"text"}}}}
//! data: {"pages":{"/page_2":null}}
//! data: {"pages":{"/page_2":{"title":"Page 2","typ":"nav-page","widget":{"text":"This is page 2.","typ":"text"}}}}
//! data: {"pages":{"/page_2":null}}
//! data: {"pages":{"/page_2":{"title":"Page 2","typ":"nav-page","widget":{"text":"This is page 2.","typ":"text"}}}}
//! ^C
//! ```
#![forbid(unsafe_code)]

use applin::action::push;
use applin::data::{Context, Rebuilder, Roster};
use applin::session::{ApplinSession, PageKey, PageMap, SessionSet};
use applin::widget::{Column, Empty, FormButton, NavPage, Text};
use servlin::reexport::permit::Permit;
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{print_log_response, socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
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
fn page_map(
    state: &Arc<ServerState>,
    rebuilder: Rebuilder<SessionState>,
) -> Result<PageMap<SessionState>, Box<dyn Error>> {
    const PAGE_2: &str = "/page_2";
    let mut keys = PageMap::new();
    // Read the `show_page_2` value and subscribe to changes.
    // Whenever the value changes, Applin calls this function
    // to rebuild the set of keys.
    let show_page_2 = *state.show_page_2.read(rebuilder);
    if show_page_2 {
        keys.add_static_page(PAGE_2, NavPage::new("Page 2", Text::new("This is page 2.")));
    }
    let state_clone = Arc::clone(state);
    keys.add_page_fn("/", move |rebuilder| {
        let show_page_2 = *state_clone.show_page_2.read(rebuilder);
        Ok(NavPage::new(
            "Dynamic Page Example",
            Column::new((
                Text::new("The page below appears and disappears every 5 seconds:"),
                if show_page_2 {
                    FormButton::new("Page 2")
                        .with_action(push(&PageKey::new(PAGE_2)))
                        .to_widget()
                } else {
                    Empty::new().to_widget()
                },
            )),
        )
        .with_poll(5))
    });
    Ok(keys)
}

fn get_or_new_session(
    state: &Arc<ServerState>,
    req: &Request,
) -> Result<Arc<ApplinSession<SessionState>>, Response> {
    let state_clone = state.clone();
    state.sessions.get_or_new(
        req,
        move |rebuilder| page_map(&state_clone, rebuilder),
        || SessionState {},
    )
}

#[allow(clippy::unnecessary_wraps)]
fn handle_req(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    match (req.method(), req.url().path()) {
        ("GET", "/health") => Ok(Response::text(200, "ok")),
        ("GET", "/") => get_or_new_session(state, req)?.poll(),
        ("GET", "/stream") => get_or_new_session(state, req)?.stream(),
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
                .write(Context::Empty)
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
