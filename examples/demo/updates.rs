use crate::{ServerState, Session};
use applin::data::Context;
use applin::session::{PageKey, PageMap};
use applin::widget::{Column, NavPage, Text};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

fn epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn start_updater_thread(state: Arc<ServerState>) {
    std::thread::spawn(move || loop {
        *state.clock_epoch_seconds.write(&Context::Empty) = epoch_seconds();
        std::thread::sleep(Duration::from_secs(1));
    });
}

pub fn add_inert_page(state: &Arc<ServerState>, keys: &mut PageMap<Session>) -> PageKey {
    let state_clone = state.clone();
    keys.add_page_fn("/updates/inert", move |rebuilder| {
        Ok(NavPage::new(
            "Inert",
            Column::new((
                Text::new(format!(
                    "epoch seconds: {}",
                    state_clone.clock_epoch_seconds.read(rebuilder)
                )),
                Text::new("The home page has poll=10, so you will see this page update when the app polls."),
            )),
        ))
    })
}

pub fn add_poll_page(state: &Arc<ServerState>, keys: &mut PageMap<Session>) -> PageKey {
    let state_clone = state.clone();
    keys.add_page_fn("/updates/poll", move |rebuilder| {
        Ok(NavPage::new(
            "Poll Every 2 Seconds",
            Column::new((
                Text::new(format!(
                    "epoch seconds: {}",
                    state_clone.clock_epoch_seconds.read(rebuilder)
                )),
                // Checkbox::new("clock-check0"),
                // Text::new("Hello"),
            )),
        )
        .with_poll(2))
    })
}

pub fn add_stream_page(state: &Arc<ServerState>, keys: &mut PageMap<Session>) -> PageKey {
    let state_clone = state.clone();
    keys.add_page_fn("/updates/stream", move |rebuilder| {
        Ok(NavPage::new(
            "Stream",
            Column::new((
                Text::new(format!(
                    "epoch seconds: {}",
                    state_clone.clock_epoch_seconds.read(rebuilder)
                )),
                // Checkbox::new("clock-check0"),
                // Text::new("Hello"),
            )),
        )
        .with_stream())
    })
}
