use crate::{ServerState, SessionState};
use applin::action::{nothing, pop, push, rpc};
use applin::session::{KeySet, PageKey};
use applin::widget::{
    AlertModal, Button, DrawerModal, Form, ModalButton, NavPage, PlainPage, Text,
};
use servlin::{Request, Response};
use std::sync::Arc;

pub static SAVE_RPC_PATH: &str = "/pages/rpc1";

pub fn save_rpc(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    session.rpc_response()
}

pub fn add_alert_page(drawer: &PageKey, keys: &mut KeySet<SessionState>) -> PageKey {
    const KEY: &str = "/pages/alert";
    let alert2 = keys.add_static_page("/pages/alert2", AlertModal::new("Alert 2").with_ok());
    keys.add_static_page(
        KEY,
        AlertModal::new("Alert Modal").with_widgets((
            ModalButton::new("Alert 2").with_action(push(&alert2)),
            ModalButton::new("Alert Modal").with_action(push(&PageKey::new(KEY))),
            ModalButton::new("Drawer Modal").with_action(push(drawer)),
            ModalButton::new("Destructive Button")
                .with_is_destructive()
                .with_action(nothing()),
            ModalButton::cancel(),
            ModalButton::new(
                "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
            )
            .with_action(nothing()),
            ModalButton::new(
                "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
            )
            .with_action(nothing()),
            ModalButton::new("Disabled Button"),
        )),
    )
}

pub fn add_drawer_modal_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/pages/drawer-modal",
        DrawerModal::new("Drawer1").with_widgets((
            ModalButton::cancel(),
            ModalButton::new("Save")
                .with_action(rpc(SAVE_RPC_PATH))
                .with_action(pop()),
        )),
    )
}

pub fn add_nav_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/pages/nav-page",
        NavPage::new(
            "Nav Page",
            Form::new((
                Text::new("Hello"), //
                Text::new("Hello 2"),
            )),
        ),
    )
}

pub fn add_plain_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/pages/plain-page",
        PlainPage::new(
            "Plain Page",
            Form::new((
                Text::new("Hello"), //
                Button::new("Back").with_action(pop()),
            )),
        ),
    )
}
