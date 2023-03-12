#![allow(dead_code)]
use serde::Serialize;
use servlin::reexport::safina_executor::Executor;
use servlin::reexport::safina_sync::Receiver;
use servlin::reexport::safina_timer;
use servlin::{socket_addr_127_0_0_1_any_port, HttpServerBuilder, Request, Response};
use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use ureq::Agent;

pub struct Messages(Mutex<Vec<serde_json::Value>>);
impl Messages {
    /// # Panics
    /// Panics when the lock is poisoned.
    pub fn pop_all(&self) -> Vec<serde_json::Value> {
        let mut all = Vec::new();
        std::mem::swap(self.0.lock().unwrap().as_mut(), &mut all);
        all
    }
}

static CLIENT_ID_COUNT: AtomicU32 = AtomicU32::new(0);

pub struct TestClient {
    pub id: u32,
    pub url: String,
    pub agent: Agent,
    pub thread_id_count: AtomicU32,
}
impl TestClient {
    pub fn new(url: &impl ToString) -> Self {
        Self {
            id: CLIENT_ID_COUNT.fetch_add(1, Ordering::AcqRel),
            url: url.to_string(),
            agent: ureq::AgentBuilder::new()
                .timeout(Duration::from_secs(5))
                .build(),
            thread_id_count: AtomicU32::new(0),
        }
    }

    #[must_use]
    pub fn name(&self) -> String {
        format!("TestClient{}", self.id)
    }

    #[must_use]
    pub fn ureq_err_to_tuple(e: ureq::Error) -> (u16, String) {
        match e {
            ureq::Error::Status(404, _) => (404, "Not Found".to_string()),
            ureq::Error::Status(code, response) => (
                code,
                response
                    .into_string()
                    .unwrap_or_else(|e| format!("error decoding response body: {e}")),
            ),
            ureq::Error::Transport(e) => (0, e.to_string()),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn json_err_to_tuple(e: std::io::Error) -> (u16, String) {
        (0, e.to_string())
    }

    /// # Errors
    /// Returns `Err` when the request fails.
    pub fn poll(&self) -> Result<serde_json::Value, (u16, String)> {
        self.get_json("/")
    }

    /// # Errors
    /// Returns `Err` when the request fails.
    pub fn get_json(&self, path: impl AsRef<str>) -> Result<serde_json::Value, (u16, String)> {
        let url = self.url.clone() + path.as_ref();
        self.agent
            .get(&url)
            .call()
            .map_err(Self::ureq_err_to_tuple)?
            .into_json::<serde_json::Value>()
            .map_err(Self::json_err_to_tuple)
    }

    /// # Errors
    /// Returns `Err` when the request fails.
    pub fn post_json(
        &self,
        path: impl AsRef<str>,
        data: impl Serialize,
    ) -> Result<serde_json::Value, (u16, String)> {
        let url = self.url.clone() + path.as_ref();
        self.agent
            .post(&url)
            .send_json(data)
            .map_err(Self::ureq_err_to_tuple)?
            .into_json::<serde_json::Value>()
            .map_err(Self::json_err_to_tuple)
    }

    /// # Errors
    /// Returns `Err` when the request fails.
    /// # Panics
    /// Panics when the server disconnects or sends malformed data.
    pub fn stream(&self) -> Result<Arc<Messages>, (u16, String)> {
        let url = self.url.clone() + "/stream";
        let response = self
            .agent
            .get(&url)
            .call()
            .map_err(Self::ureq_err_to_tuple)?;
        assert_eq!("text/event-stream", response.content_type());
        let reader = BufReader::new(response.into_reader());
        let messages = Arc::new(Messages(Mutex::new(Vec::new())));
        let messages2 = Arc::clone(&messages);
        let thread_id = self.thread_id_count.fetch_add(1, Ordering::AcqRel);
        let prefix = format!("{} stream{}", self.name(), thread_id);
        std::thread::spawn(move || {
            for line_result in reader.lines() {
                println!("{} {:?}", &prefix, &line_result);
                let line = line_result.unwrap();
                if line.is_empty() {
                    continue;
                }
                if let ("data", data) = line.split_once(':').unwrap() {
                    let value: serde_json::Value = serde_json::from_str(data).unwrap();
                    messages2.0.lock().unwrap().push(value);
                } else {
                    println!("{} got unexpected message from server: {:?}", &prefix, line);
                }
            }
            println!("{} finished", &prefix);
        });
        Ok(messages)
    }
}

/// # Panics
/// Panics when it fails to start the server.
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
    let url = format!("http://{addr }");
    (url, receiver)
}
