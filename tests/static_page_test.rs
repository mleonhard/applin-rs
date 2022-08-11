use applin::data::Rebuilder;
use applin::session::{KeySet, SessionSet};
use applin::widget::{NavPage, Text};
use serde_json::{json, Value};
use servlin::reexport::safina_executor::Executor;
use servlin::reexport::safina_sync::Receiver;
use servlin::reexport::safina_timer;
use servlin::{socket_addr_127_0_0_1_any_port, HttpServerBuilder, Request, Response};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use ureq::Agent;

pub fn start_for_test<T, KeySetFn, ReqHandlerFn>(
    key_set_fn: KeySetFn,
    req_handler: ReqHandlerFn,
) -> (Arc<Executor>, String, Receiver<()>, Agent)
where
    T: Default + 'static + Send + Sync,
    KeySetFn: Fn(Rebuilder<T>) -> Result<KeySet<T>, Box<dyn std::error::Error>>
        + 'static
        + Clone
        + Send
        + Sync,
    ReqHandlerFn: FnOnce(Request) -> Result<Response, Response> + 'static + Clone + Send + Sync,
{
    safina_timer::start_timer_thread();
    let executor = Executor::new(1, 1).unwrap();
    let sessions: Arc<SessionSet<T>> = Arc::new(SessionSet::new(&executor));
    let handler_inner = move |req: Request| match (req.method(), req.url().path()) {
        ("GET", "/") => sessions
            .get_or_new(&req, key_set_fn, || T::default())?
            .poll(),
        ("GET", "/stream") => sessions
            .get_or_new(&req, key_set_fn, || T::default())?
            .stream(),
        _ => req_handler(req),
    };
    let handler = move |req: Request| handler_inner(req).unwrap_or_else(|r| r);
    let (addr, receiver): (SocketAddr, Receiver<()>) = executor
        .block_on(
            HttpServerBuilder::new()
                .listen_addr(socket_addr_127_0_0_1_any_port())
                .spawn(handler),
        )
        .unwrap();
    let url = format!("http://{}", addr);
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(5))
        .build();
    (executor, url, receiver, agent)
}

pub trait UreqErrorUnwrapStatus {
    fn unwrap_status(self) -> (u16, ureq::Response);
}
impl UreqErrorUnwrapStatus for ureq::Error {
    fn unwrap_status(self) -> (u16, ureq::Response) {
        match self {
            ureq::Error::Status(code, response) => (code, response),
            ureq::Error::Transport(t) => {
                panic!("unwrap_status called on {:?}", ureq::Error::Transport(t))
            }
        }
    }
}

#[test]
pub fn static_page_test() {
    println!("{:?}", url::Url::parse("http://host//path"));
    let (_executor, url, _receiver, agent) = start_for_test(
        |_: Rebuilder<()>| {
            Ok(KeySet::new().with_static_page("/", NavPage::new("t1", Text::new("hello1"))))
        },
        move |_req| Err(Response::not_found_404()),
    );
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
            .get(&url)
            .call()
            .unwrap()
            .into_json::<Value>()
            .unwrap()
    );
    assert_eq!(
        404,
        agent
            .get(&(url.clone() + "/nonexistent"))
            .call()
            .unwrap_err()
            .unwrap_status()
            .0
    );
    // TODO: Uncomment test and fix bug.
    // assert_eq!(
    //     404,
    //     agent
    //         .get(&(url.clone() + "//nonexistent"))
    //         .call()
    //         .unwrap_err()
    //         .unwrap_status()
    //         .0
    // );
}
