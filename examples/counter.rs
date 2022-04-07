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
use maggie::app::App;
use maggie::rebuilder::Rebuilder;
use maggie::rebuilder_set::RebuilderSet;
use maggie::session::{session_not_found, Session};
use maggie::session_cookie::SessionCookie;
use maggie::widgets::{text, Button, DetailCell, TitleBar};
use serde_json::json;
use std::collections::HashMap;
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

    pub fn get(&self, rebuilder: Rebuilder) -> u64 {
        self.rebuilders.insert(rebuilder);
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
    user_states: RwLock<HashMap<SessionCookie, Arc<UserState>>>,
}
impl State {
    pub fn new(executor: &Arc<Executor>) -> Self {
        Self {
            global_counter: Counter::new(executor),
            user_states: RwLock::new(HashMap::new()),
        }
    }
}

fn app(state: &Arc<State>, _rebuilder: Rebuilder) -> Result<App, Box<dyn Error>> {
    let mut app = App::new();
    app.add_static(
        "/",
        json!([
            TitleBar::new("Counter Example"),
            DetailCell::new("Global Counter").with_action("/global_counter"),
            DetailCell::new("User Counter").with_action("/user_counter"),
        ]),
    );
    let state_clone = state.clone();
    app.add_fn("/global_counter", move |rebuilder| {
        Ok(json!([
            TitleBar::new("Global Counter").with_back(),
            text(format!(
                "Value: {}",
                state_clone.global_counter.get(rebuilder)
            )),
            Button::new("Increment").with_action("rpc:/global_increment"),
        ]))
    });
    let state_clone = state.clone();
    app.add_fn("/user_counter", move |rebuilder| {
        let count = state_clone
            .user_states
            .read()
            .unwrap()
            .get(&rebuilder.session()?.cookie)
            .ok_or_else(|| "session not found")?
            .count
            .load(Ordering::Acquire);
        Ok(json!([
            TitleBar::new("My Counter").with_back(),
            text(format!("Value: {}", count)),
            Button::new("Increment").with_action("rpc:/user_increment"),
        ]))
    });
    Ok(app)
}

#[allow(clippy::unnecessary_wraps)]
fn maggie(state: &Arc<State>, _req: &Request) -> Result<Response, Response> {
    // TODO: Look for existing session.
    let cookie = SessionCookie::new_random();
    let state_clone = state.clone();
    let (session, response) = Session::new(move |rebuilder| app(&state_clone, rebuilder));
    let user_state = Arc::new(UserState::new(session.clone()));
    state
        .user_states
        .write()
        .unwrap()
        .insert(cookie, user_state);
    session.schedule_rebuild_keys(None);
    Ok(response)
}

fn global_increment(state: &Arc<State>, req: &Request) -> Result<Response, Response> {
    let cookie = SessionCookie::from_req(req)?;
    let user_state = state
        .user_states
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
        .user_states
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
