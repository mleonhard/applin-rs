use serde::Serialize;
use serde_json::Value;
use servlin::reexport::safina_executor::Executor;
use servlin::reexport::safina_sync::Receiver;
use servlin::reexport::safina_timer;
use servlin::{socket_addr_127_0_0_1_any_port, HttpServerBuilder, Request, Response};
use std::fmt::Debug;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use ureq::Agent;

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
#[allow(dead_code)]
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
    fn post_json(&self, path: &str, data: impl Serialize) -> Result<Value, UreqError>;
}
impl UreqJsonHelper for ureq::Agent {
    fn get_json(&self, path: &str) -> Result<Value, UreqError> {
        self.get(path)
            .call()
            .map_err(|e| match e {
                ureq::Error::Status(n, _) => UreqError::Status(n),
                ureq::Error::Transport(e) => UreqError::Other(e.to_string()),
            })?
            .into_json::<Value>()
            .map_err(|e| UreqError::Other(e.to_string()))
    }

    fn post_json(&self, path: &str, data: impl Serialize) -> Result<Value, UreqError> {
        self.post(path)
            .send_json(data)
            .map_err(|e| match e {
                ureq::Error::Status(400, response) => UreqError::Other(format!(
                    "400 {:?}",
                    response
                        .into_string()
                        .unwrap_or_else(|e| format!("error decoding request body: {}", e))
                )),
                ureq::Error::Status(n, _) => UreqError::Status(n),
                ureq::Error::Transport(e) => UreqError::Other(e.to_string()),
            })?
            .into_json::<Value>()
            .map_err(|e| UreqError::Other(e.to_string()))
    }
}
