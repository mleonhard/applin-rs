use applin::data::Rebuilder;
use applin::session::{KeySet, SessionSet};
use applin::widget::{NavPage, Text};
use serde_json::{json, Value};
use servlin::reexport::safina_executor::Executor;
use servlin::reexport::safina_sync::Receiver;
use servlin::reexport::safina_timer;
use servlin::{socket_addr_127_0_0_1_any_port, HttpServerBuilder, Request, Response};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use ureq::{Agent, Error};

pub fn new_agent() -> Agent {
    ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(5))
        .build()
}

pub fn start_for_test<F>(executor: &Arc<Executor>, req_handler: F) -> (String, Receiver<()>)
where
    F: FnOnce(Request) -> Result<Response, Response> + 'static + Clone + Send + Sync,
{
    safina_timer::start_timer_thread();
    let (addr, receiver): (SocketAddr, Receiver<()>) = executor
        .block_on(
            HttpServerBuilder::new()
                .listen_addr(socket_addr_127_0_0_1_any_port())
                .spawn(move |req| req_handler(req).unwrap_or_else(|r| r)),
        )
        .unwrap();
    let url = format!("http://{}", addr);
    (url, receiver)
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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UreqError {
    Status(u16),
    Other(String),
}
impl UreqError {
    pub fn unwrap_status(self) -> u16 {
        match self {
            UreqError::Status(n) => n,
            UreqError::Other(s) => panic!("unwrap_status called on UreqError::Other({:?})", s),
        }
    }

    pub fn unwrap_other(self) -> String {
        match self {
            UreqError::Status(n) => panic!("unwrap_status called on UreqError::Status({})", n),
            UreqError::Other(s) => s,
        }
    }
}

pub trait UreqJsonHelper {
    fn get_json(&self, path: &str) -> Result<Value, UreqError>;
}
impl UreqJsonHelper for ureq::Agent {
    fn get_json(&self, path: &str) -> Result<Value, UreqError> {
        self.get(path)
            .call()
            .map_err(|e| match e {
                Error::Status(n, _) => UreqError::Status(n),
                Error::Transport(e) => UreqError::Other(e.to_string()),
            })?
            .into_json::<Value>()
            .map_err(|e| UreqError::Other(e.to_string()))
    }
}

#[test]
pub fn static_page() {
    let executor = Executor::new(1, 1).unwrap();
    let sessions = Arc::new(SessionSet::new(&executor));
    let key_set_fn =
        |_| Ok(KeySet::new().with_static_page("/", NavPage::new("t1", Text::new("hello1"))));
    let req_handler = move |req: Request| match (req.method.as_str(), req.url.path()) {
        ("GET", "/") => sessions.get_or_new(&req, key_set_fn, || ())?.poll(),
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    assert_eq!(
        json!({
            "pages": {"/": {
                "typ": "nav-page",
                "title": "t1",
                "widget": {"typ":"text", "text": "hello1"},
            }},
            "vars": null,
        }),
        new_agent().get_json(&url).unwrap()
    );
    assert_eq!(
        Err(UreqError::Status(404)),
        new_agent().get_json(&(url.clone() + "/nonexistent"))
    );
    // TODO: Uncomment test and fix bug.
    // assert_eq!(
    //     404,
    //     new_agent()
    //         .get(&(url.clone() + "//nonexistent"))
    //         .call()
    //         .unwrap_err()
    //         .unwrap_status()
    //         .0
    // );
}

#[test]
pub fn user_specific_static_page() {
    static ID_COUNTER: AtomicU32 = AtomicU32::new(3);
    struct UserState {
        id: u32,
    }
    impl UserState {
        fn new() -> Self {
            Self {
                id: ID_COUNTER.fetch_add(1, Ordering::AcqRel),
            }
        }
    }
    let executor = Executor::new(1, 1).unwrap();
    let sessions: Arc<SessionSet<UserState>> = Arc::new(SessionSet::new(&executor));
    let req_handler = move |req: Request| match (req.method.as_str(), req.url.path()) {
        ("GET", "/") => {
            let key_set_fn = |rebuilder: Rebuilder<UserState>| {
                let id = rebuilder.session()?.state().id;
                Ok(KeySet::new()
                    .with_static_page("/", NavPage::new("t1", Text::new(format!("hello {}", id)))))
            };
            sessions
                .get_or_new(&req, key_set_fn, UserState::new)?
                .poll()
        }
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    let agent1 = new_agent();
    let agent2 = new_agent();
    assert_eq!(
        json!({
            "pages": {"/": {
                "typ": "nav-page",
                "title": "t1",
                "widget": {"typ":"text", "text": "hello 3"},
            }},
            "vars": null,
        }),
        agent1.get_json(&url).unwrap()
    );
    assert_eq!(
        json!({
            "pages": {"/": {
                "typ": "nav-page",
                "title": "t1",
                "widget": {"typ":"text", "text": "hello 4"},
            }},
            "vars": null,
        }),
        agent2.get_json(&url).unwrap()
    );
    assert_eq!(
        json!({
            "pages": {},
            "vars": null,
        }),
        agent1.get_json(&url).unwrap()
    );
}

#[test]
pub fn user_specific_key_set() {
    static ID_COUNTER: AtomicU32 = AtomicU32::new(3);
    struct UserState {
        id: u32,
    }
    impl UserState {
        fn new() -> Self {
            Self {
                id: ID_COUNTER.fetch_add(1, Ordering::AcqRel),
            }
        }
    }
    let executor = Executor::new(1, 1).unwrap();
    let sessions: Arc<SessionSet<UserState>> = Arc::new(SessionSet::new(&executor));
    let req_handler = move |req: Request| match (req.method.as_str(), req.url.path()) {
        ("GET", "/") => {
            let key_set_fn = |rebuilder: Rebuilder<UserState>| {
                let id = rebuilder.session()?.state().id;
                Ok(KeySet::new().with_static_page(
                    format!("/user{}", id),
                    NavPage::new("t1", Text::new(format!("hello {}", id))),
                ))
            };
            sessions
                .get_or_new(&req, key_set_fn, UserState::new)?
                .poll()
        }
        _ => Ok(Response::not_found_404()),
    };
    let (url, _receiver) = start_for_test(&executor, req_handler);
    let agent1 = new_agent();
    let agent2 = new_agent();
    assert_eq!(
        json!({
            "pages": {"/user3": {
                "typ": "nav-page",
                "title": "t1",
                "widget": {"typ":"text", "text": "hello 3"},
            }},
            "vars": null,
        }),
        agent1.get_json(&url).unwrap()
    );
    assert_eq!(
        json!({
            "pages": {"/user4": {
                "typ": "nav-page",
                "title": "t1",
                "widget": {"typ":"text", "text": "hello 4"},
            }},
            "vars": null,
        }),
        agent2.get_json(&url).unwrap()
    );
    assert_eq!(
        json!({
            "pages": {},
            "vars": null,
        }),
        agent1.get_json(&url).unwrap()
    );
}
