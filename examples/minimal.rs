//! Minimal Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package applin --example minimal
//! Access the app with an Applin client at http://127.0.0.1:8000/
//! ```
//!
//! Make a request to the server
//! ```
//! $ curl --include http://127.0.0.1:8000/
//! HTTP/1.1 200 OK
//! content-type: application/json; charset=UTF-8
//! content-length: 101
//! cache-control: no-store
//! set-cookie: session=3809352776013307961-9190653589617809531; HttpOnly; Max-Age=2592000; SameSite=Strict
//!
// {"pages":{"/":{"title":"Minimal Example","typ":"plain-page","widget":{"text":"Hello","typ":"text"}}}}
//! ```
#![forbid(unsafe_code)]

use applin::data::Rebuilder;
use applin::session::{PageMap, SessionSet};
use applin::widget::{NavPage, Text};
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
use std::sync::Arc;

pub fn main() {
    println!("Access the app with an Applin client at http://127.0.0.1:8000/");
    safina_timer::start_timer_thread();
    let executor = safina_executor::Executor::default();
    let sessions: Arc<SessionSet<()>> = Arc::new(SessionSet::new(&executor));
    let page_map_fn = move |_rebuilder: Rebuilder<()>| {
        Ok(PageMap::new().with_static_page(
            "/",
            NavPage::new("Minimal Example", Text::new("Hello")).with_poll(10),
        ))
    };
    let session_state_fn = move || ();
    let req_handler = move |req: Request| match (req.method(), req.url().path()) {
        ("GET", "/") => sessions
            .get_or_new(&req, page_map_fn, session_state_fn)?
            .poll(),
        ("GET", "/stream") => sessions
            .get_or_new(&req, page_map_fn, session_state_fn)?
            .stream(),
        _ => Ok(Response::text(404, "Not found")),
    };
    executor
        .block_on(
            HttpServerBuilder::new()
                .listen_addr(socket_addr_127_0_0_1(8000))
                .spawn_and_join(move |req: Request| {
                    req_handler(req).unwrap_or_else(|response| response)
                }),
        )
        .unwrap();
}
