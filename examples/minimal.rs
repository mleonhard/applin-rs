//! Minimal Example
//! =================
//!
//! Start the server:
//! ```
//! $ cargo run --package maggie --example minimal
//! Access the app with a Maggie client at http://127.0.0.1:8000/
//! ```
//!
//! Make a request to the server
//! ```
//! $ curl -v http://127.0.0.1:8000/
//! *   Trying 127.0.0.1...
//! * TCP_NODELAY set
//! * Connected to 127.0.0.1 (127.0.0.1) port 8000 (#0)
//! > GET / HTTP/1.1
//! > Host: 127.0.0.1:8000
//! > User-Agent: curl/7.64.1
//! > Accept: */*
//! >
//! < HTTP/1.1 200 OK
//! < content-type: text/event-stream
//! < transfer-encoding: chunked
//! < set-cookie: session=7324714364658396595-8735568333978848934; HttpOnly; Max-Age=2592000; SameSite=Strict; Secure
//! <
//! data: {"pages":{"/":{"title":"Dynamic Page Example","typ":"plain-page","widget":{"text":"Hello","typ":"text"}}}}
//! ^C
//! ```
#![forbid(unsafe_code)]

use beatrice::reexport::{safina_executor, safina_timer};
use beatrice::{socket_addr_127_0_0_1, HttpServerBuilder, Request};
use maggie::context::Context;
use maggie::key_set::KeySet;
use maggie::pages::PlainPage;
use maggie::session_set::SessionSet;
use maggie::widgets::Text;
use std::sync::Arc;

pub fn main() {
    println!("Access the app with a Maggie client at http://127.0.0.1:8000/");
    safina_timer::start_timer_thread();
    let executor = safina_executor::Executor::default();
    let sessions: Arc<SessionSet<()>> = Arc::new(SessionSet::new(&executor));
    let key_set_fn = move |_ctx: &Context<()>| {
        Ok(KeySet::new().with_static_page(
            "/",
            PlainPage::new("Dynamic Page Example", Text::new("Hello")),
        ))
    };
    let session_state_fn = move || ();
    let req_handler =
        move |req: Request| match sessions.resume_or_new(&req, key_set_fn, session_state_fn) {
            Ok((_session, response)) => response,
            Err(response) => response,
        };
    executor
        .block_on(
            HttpServerBuilder::new()
                .listen_addr(socket_addr_127_0_0_1(8000))
                .spawn_and_join(req_handler),
        )
        .unwrap();
}
