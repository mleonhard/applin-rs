//! Counter Example
//! =================
//!
//! Start the server:
//! ```
//! % cargo run --package maggie --example counter
//! Access the server at http://127.0.0.1:8000/
//! ^C
//! ```
//!
//! Access the app with a Maggie client:
//! <http://127.0.0.1:8000/>
#![forbid(unsafe_code)]

use crate::safina_executor::Executor;
use beatrice::reexport::{safina_executor, safina_timer};
use beatrice::{print_log_response, socket_addr_127_0_0_1, HttpServerBuilder, Request, Response};
use core::sync::atomic::{AtomicU64, Ordering};
use maggie::rebuilder::Rebuilder;
use maggie::rebuilder_set::RebuilderSet;
use maggie::session::{session_not_found, Session};
use maggie::session_cookie::SessionCookie;
use maggie::widgets::{text, Button, DetailCell, TitleBar};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::{Arc, RwLock};

struct Counter {
    value: AtomicU64,
    rebuilders: RebuilderSet,
}
impl Counter {
    pub fn new(executor: &Arc<Executor>) -> Self {
        Self {
            value: AtomicU64::new(0),
            rebuilders: RebuilderSet::new(executor),
        }
    }

    pub fn get(&self, rebuilder: &Rebuilder) -> u64 {
        self.rebuilders.insert(rebuilder.clone());
        self.value.load(Ordering::Acquire)
    }

    pub fn increment(&self, rpc_session: Option<&Arc<Session>>) {
        self.value.fetch_add(1, Ordering::AcqRel);
        self.rebuilders.rebuild_all(rpc_session);
    }

    // pub fn add(&self, n: u64, rpc_session: Option<&Arc<Session>>) {
    //     self.value.fetch_add(n, Ordering::AcqRel);
    //     self.rebuilders.rebuild_all(rpc_session);
    // }
}

// struct UserId(pub u64);
// impl UserId {
//     pub fn new_random() -> Self {
//         Self(random_u64())
//     }
// }
// impl Debug for UserId {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "UserId({})", self.0)
//     }
// }

struct UserState {
    pub session: Arc<Session>,
    pub count: AtomicU64,
}
impl UserState {
    pub fn new(session: Arc<Session>) -> Self {
        Self {
            session,
            count: AtomicU64::new(0),
        }
    }
}

struct State {
    global_counter: Counter,
    sessions: RwLock<HashMap<SessionCookie, Arc<UserState>>>,
}
impl State {
    pub fn new(executor: &Arc<Executor>) -> Self {
        Self {
            global_counter: Counter::new(executor),
            sessions: RwLock::new(HashMap::new()),
        }
    }

    pub fn keys(
        self: &Arc<Self>,
        _rebuilder: &Rebuilder,
    ) -> Result<HashSet<String>, Box<dyn Error>> {
        Ok(["/", "/global_counter", "/user_counter"]
            .into_iter()
            .map(str::to_string)
            .collect())
    }

    pub fn value(
        self: &Arc<Self>,
        session_cookie: SessionCookie,
        key: &str,
        rebuilder: &Rebuilder,
    ) -> Result<Value, Box<dyn Error>> {
        match key {
            "/" => Ok(json!([
                TitleBar::new("Counter Example"),
                DetailCell::new("Global Counter").with_action("/global_counter"),
                DetailCell::new("User Counter").with_action("/user_counter"),
            ])),
            "/global_counter" => Ok(json!([
                TitleBar::new("Global Counter").with_back(),
                text(format!("Value: {}", self.global_counter.get(rebuilder))),
                Button::new("Increment").with_action("rpc:/global_increment"),
            ])),
            "/user_counter" => {
                let count = self
                    .sessions
                    .read()
                    .unwrap()
                    .get(&session_cookie)
                    .ok_or_else(|| "session not found")?
                    .count
                    .load(Ordering::Acquire);
                Ok(json!([
                    TitleBar::new("My Counter").with_back(),
                    text(format!("Value: {}", count)),
                    Button::new("Increment").with_action("rpc:/user_increment"),
                ]))
            }
            _ => Err(format!("unknown key: {:?}", key).into()),
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
fn maggie(state: &Arc<State>, _req: &Request) -> Result<Response, Response> {
    // TODO: Look for existing session.
    let cookie = SessionCookie::new_random();
    let state_clone1 = state.clone();
    let state_clone2 = state.clone();
    let (session, response) = Session::new(
        cookie,
        move |rebuilder: &Rebuilder| state_clone1.keys(rebuilder),
        move |path: &str, rebuilder: &Rebuilder| state_clone2.value(cookie, path, rebuilder),
    );
    let user_state = Arc::new(UserState::new(session));
    state.sessions.write().unwrap().insert(cookie, user_state);
    Ok(response)
}

fn global_increment(state: &Arc<State>, req: &Request) -> Result<Response, Response> {
    let cookie = SessionCookie::from_req(req)?;
    let user_state = state
        .sessions
        .read()
        .unwrap()
        .get(&cookie)
        .cloned()
        .ok_or_else(session_not_found)?;
    state.global_counter.increment(Some(&user_state.session));
    user_state.session.response()
}

fn user_increment(state: &Arc<State>, req: &Request) -> Result<Response, Response> {
    let cookie = SessionCookie::from_req(req)?;
    let user_state = state
        .sessions
        .read()
        .unwrap()
        .get(&cookie)
        .cloned()
        .ok_or_else(session_not_found)?;
    user_state.count.fetch_add(1, Ordering::AcqRel);
    user_state
        .session
        .schedule_rebuild_value("/user_counter", Some(&user_state.session));
    user_state.session.response()
}

// fn add(state: &Arc<State>, req: &Request) -> Result<Response, Response> {
//     let cookie = SessionCookie::from_req(req)?;
//     let user_state = state
//         .sessions
//         .read()
//         .unwrap()
//         .get(&cookie)
//         .cloned()
//         .ok_or_else(session_not_found)?;
//     #[derive(Deserialize)]
//     struct Input {
//         num: u64,
//     }
//     let input: Input = req.json()?;
//     let num = if input.num > 5 {
//         return Err(user_error("num is too big"));
//     } else {
//         input.num
//     };
//     user_state.count.fetch_add(num, Ordering::AcqRel);
//     user_state
//         .session
//         .schedule_rebuild_value("/user_counter", Some(&user_state.session));
//     user_state.session.response()
// }

#[allow(clippy::unnecessary_wraps)]
fn handle_req(state: &Arc<State>, req: &Request) -> Result<Response, Response> {
    match (req.method(), req.url().path()) {
        ("GET", "/health") => Ok(Response::text(200, "ok")),
        ("GET", "/maggie") => maggie(state, req),
        ("POST", "/global_increment") => global_increment(state, req),
        ("POST", "/user_increment") => user_increment(state, req),
        // ("POST", "/add") => add(state, req),
        _ => Ok(Response::text(404, "Not found")),
    }
}

pub fn main() {
    println!("Access the app at http://127.0.0.1:8000/maggie");
    safina_timer::start_timer_thread();
    let executor = safina_executor::Executor::default();
    let state = Arc::new(State::new(&executor));
    let request_handler = move |req: Request| {
        print_log_response(
            req.method().to_string(),
            req.url().clone(),
            handle_req(&state, &req),
        )
    };
    executor
        .block_on(
            HttpServerBuilder::new()
                .listen_addr(socket_addr_127_0_0_1(8000))
                .max_conns(100)
                .spawn_and_join(request_handler),
        )
        .unwrap();
}
