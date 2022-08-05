use applin::session::{KeySet, SessionSet};
use applin::widget::{NavPage, Text};
use serde_json::{json, Value};
use servlin::reexport::safina_sync::Receiver;
use servlin::reexport::{safina_executor, safina_timer};
use servlin::{socket_addr_127_0_0_1_any_port, HttpServerBuilder, Request};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

#[test]
pub fn static_page_test() {
    safina_timer::start_timer_thread();
    let executor = safina_executor::Executor::new(1, 1).unwrap();
    let sessions: Arc<SessionSet<()>> = Arc::new(SessionSet::new(&executor));
    let key_set_fn =
        |_| Ok(KeySet::new().with_static_page("/", NavPage::new("t1", Text::new("hello1"))));
    let req_handler = move |req: Request| match sessions.get_or_new(&req, key_set_fn, || ()) {
        Ok(session) => session.poll().unwrap_or_else(|response| response),
        Err(response) => response,
    };
    let (addr, _receiver): (SocketAddr, Receiver<()>) = executor
        .block_on(
            HttpServerBuilder::new()
                .listen_addr(socket_addr_127_0_0_1_any_port())
                .spawn(req_handler),
        )
        .unwrap();
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(5))
        .build();
    assert_eq!(
        json!({
            "pages": {"/": {
                "typ": "nav-page",
                "title": "t1",
                "widget": {"typ":"text", "text": "hello1"},
            }},
            "vars": null,
        }),
        agent
            .get(&format!("http://{}/", addr))
            .call()
            .unwrap()
            .into_json::<Value>()
            .unwrap()
    );
}
