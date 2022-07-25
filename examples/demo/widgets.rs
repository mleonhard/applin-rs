use crate::{ServerState, SessionState};
use applin::action::{nothing, pop, push, rpc};
use applin::session::{KeySet, PageKey};
use applin::widget::{
    AlertModal, BackButton, Button, Column, Empty, Form, FormDetail, NavPage, Text,
};
use servlin::{Request, Response};
use std::sync::Arc;

// TODO: Move these to different files.
pub static BACK_RPC_PATH: &str = "/widgets/back";

pub fn back_rpc(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    session.rpc_response()
}

pub fn add_back_button_pages(keys: &mut KeySet<SessionState>) -> PageKey {
    let default = keys.add_static_page(
        "/back-button-default",
        NavPage::new("Default", Empty::new()),
    );
    let disabled = keys.add_static_page(
        "/back-button-disabled",
        NavPage::new(
            "Disabled",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(BackButton::new()),
    );
    let missing = keys.add_static_page(
        "/back-button-missing",
        NavPage::new(
            "Missing",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .without_back(),
    );
    let rpc_ok = keys.add_static_page(
        "/back-button-rpc-ok",
        NavPage::new(
            "RPC",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(
            BackButton::new()
                .with_action(rpc(BACK_RPC_PATH))
                .with_action(pop()),
        ),
    );
    let rpc_err = keys.add_static_page(
        "/back-button-rpc-error",
        NavPage::new(
            "RPC Error",
            Column::new((Button::new("Back").with_action(pop()),)),
        )
        .with_start(
            BackButton::new()
                .with_action(rpc("/nonexistent-method"))
                .with_action(pop()),
        ),
    );
    keys.add_static_page(
        "/back-button",
        NavPage::new(
            "Back Button",
            Form::new((
                FormDetail::new("Default").with_action(push(&default)),
                FormDetail::new("Disabled").with_action(push(&disabled)),
                FormDetail::new("Missing").with_action(push(&missing)),
                FormDetail::new("RPC").with_action(push(&rpc_ok)),
                FormDetail::new("RPC Error").with_action(push(&rpc_err)),
            )),
        ),
    )
}

pub fn add_button_page(keys: &mut KeySet<SessionState>) -> PageKey {
    let pressed = keys.add_static_page(
        "/button-pressed",
        AlertModal::new("Button Pressed").with_ok(),
    );
    keys.add_static_page(
        "/button",
        NavPage::new(
            "Button",
            Form::new((
                Button::new("Button").with_action(push(&pressed)),
                Button::new(
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                )
                .with_action(push(&pressed)),
                Button::new(
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                )
                .with_action(push(&pressed)),
                Text::new("Button with empty label:"),
                Button::new("").with_action(push(&pressed)),
                Button::new("Disabled Button"),
                Button::new("Does Nothing").with_action(nothing()),
            )),
        ),
    )
}
