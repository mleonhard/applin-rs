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
use maggie::context::Context;
use maggie::key_set::KeySet;
use maggie::random::random_u64;
use maggie::roster::Roster;
use maggie::session_set::SessionSet;
use maggie::widgets::{text, Button, DetailCell, TitleBar};
use serde_json::Value;
use std::error::Error;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

struct UserId(pub u64);
impl UserId {
    pub fn new_random() -> Self {
        Self(random_u64())
    }
}
impl Debug for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "UserId({})", self.0)
    }
}

struct UserState {
    pub user_id: UserId,
    pub count: Roster<u64, Self>,
}
impl UserState {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            count: Roster::new(0),
        }
    }
}

struct State {
    global_counter: Roster<u64, UserState>,
    sessions: SessionSet<UserState>,
}
impl State {
    pub fn new(executor: &Arc<Executor>) -> Self {
        Self {
            global_counter: Roster::new(0).with_cleanup_task(executor),
            sessions: SessionSet::new(),
        }
    }
}

fn key_set(
    state: &Arc<State>,
    ctx: &Context<UserState>,
) -> Result<KeySet<UserState>, Box<dyn Error>> {
    let mut app = KeySet::new();
    app.add_static(
        "/",
        Value::Array(vec![
            TitleBar::new("Counter Example").build(),
            DetailCell::new("Global Counter")
                .with_action("/global_counter")
                .build(),
            DetailCell::new("User Counter")
                .with_action("/user_counter")
                .build(),
        ]),
    );
    let state_clone = state.clone();
    app.add_fn("/global_counter", move |ctx| {
        //dbg!("/global_counter");
        Ok(Value::Array(vec![
            TitleBar::new("Global Counter").with_back().build(),
            text(format!("Value: {}", *state_clone.global_counter.read(&ctx))),
            Button::new("Increment")
                .with_action("rpc:/global_increment")
                .build(),
            if *state_clone.global_counter.read(&ctx) > 5 {
                DetailCell::new("Global Counter High")
                    .with_action("/high_global_counter")
                    .build()
            } else {
                Value::Null
            },
        ]))
    });
    app.add_fn("/user_counter", move |ctx| {
        //dbg!("/user_counter");
        Ok(Value::Array(vec![
            TitleBar::new("My Counter").with_back().build(),
            text(format!(
                "Value: {}",
                *ctx.session()?.value().count.read(&ctx)
            )),
            Button::new("Increment")
                .with_action("rpc:/user_increment")
                .build(),
        ]))
    });
    if *state.global_counter.read(ctx) > 5 {
        app.add_static(
            "/high_global_counter",
            Value::Array(vec![
                TitleBar::new("High Global Counter").with_back().build(),
                text("The global counter is higher than 5."),
            ]),
        );
    }
    //dbg!(&app);
    Ok(app)
}

#[allow(clippy::unnecessary_wraps)]
fn maggie(state: &Arc<State>, req: &Request) -> Result<Response, Response> {
    let state_clone = state.clone();
    let (_session, response) = state.sessions.resume_or_new(
        req,
        move |ctx| key_set(&state_clone, ctx),
        || UserState::new(UserId::new_random()),
    )?;
    //dbg!(&response);
    Ok(response)
}

fn global_increment(state: &Arc<State>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    (*state.global_counter.write(Some(session.id()))) += 1;
    session.rpc_response()
}

fn user_increment(state: &Arc<State>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    (*session.value().count.write(Some(session.id()))) += 1;
    session.rpc_response()
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
