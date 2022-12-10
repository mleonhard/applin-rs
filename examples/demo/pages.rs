use crate::{SessionState, OK_RPC_PATH};
use applin::action::{pop, push, rpc};
use applin::session::{KeySet, PageKey};
use applin::widget::{
    AlertModal, Button, DrawerModal, Form, ModalButton, NavPage, PlainPage, Text,
};

pub fn add_alert_page(drawer: &PageKey, keys: &mut KeySet<SessionState>) -> PageKey {
    const KEY: &str = "/pages/alert";
    let button_pressed_modal = keys.add_static_page(
        "/pages/button-pressed",
        AlertModal::new("Button Pressed").with_ok(),
    );
    keys.add_static_page(
        KEY,
        AlertModal::new("Alert Modal").with_widgets((
            ModalButton::new("Button").with_action(push(&button_pressed_modal)),
            ModalButton::new(
                "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
            )
            .with_action(push(&button_pressed_modal)),
            ModalButton::new(
                "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
            )
            .with_action(push(&button_pressed_modal)),
            ModalButton::new("Destructive Button")
                .with_is_destructive()
                .with_action(push(&button_pressed_modal)),
            ModalButton::new("Disabled Button"),
            ModalButton::new("Show Drawer Modal").with_action(push(drawer)),
            ModalButton::cancel(),
        )),
    )
}

pub fn add_drawer_modal_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/pages/drawer-modal",
        DrawerModal::new("Drawer1").with_widgets((
            ModalButton::cancel(),
            ModalButton::new("Save")
                .with_action(rpc(OK_RPC_PATH))
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
