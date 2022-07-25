use crate::{ServerState, SessionState};
use applin::action::{nothing, pop, push, rpc};
use applin::session::{KeySet, PageKey};
use applin::widget::{
    AlertModal, BackButton, Button, Column, Empty, Form, FormButton, FormCheckbox, FormDetail,
    FormError, FormSection, NavPage, Text,
};
use servlin::{Request, Response};
use std::sync::Arc;

// TODO: Move these to different files.
pub static BACK_RPC_PATH: &str = "/widgets/back";
pub static FORM_CHECKBOX_RPC_PATH: &str = "/widgets/checkbox";

pub fn back_rpc(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    session.rpc_response()
}

pub fn form_checkbox_rpc(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
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

pub fn add_form_button_page(keys: &mut KeySet<SessionState>) -> PageKey {
    let pressed = keys.add_static_page(
        "/form-button-pressed",
        AlertModal::new("Form Button Pressed").with_ok(),
    );
    keys.add_static_page(
        "/form-button",
        NavPage::new(
            "Form Button",
            Form::new((
                FormButton::new("Button1").with_action(push(&pressed)),
                FormButton::new("").with_action(push(&pressed)),
                FormButton::new(
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                )
                .with_action(push(&pressed)),
                FormButton::new(
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                )
                .with_action(push(&pressed)),
                FormButton::new("Disabled"),
                FormButton::new("Does Nothing").with_action(nothing()),
            )),
        ),
    )
}

pub fn add_form_checkbox_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-checkbox",
        NavPage::new(
            "Form Checkbox",
            Form::new((
                FormCheckbox::new("checkbox", "Checkbox"),
                FormCheckbox::new("initial-checked", "Initially checked").with_initial(true),
                FormCheckbox::new("with-rpc", "Does RPC on change")
                    .with_rpc(FORM_CHECKBOX_RPC_PATH),
                FormCheckbox::new("with-bad-rpc", "Does RPC on change, but it fails")
                    .with_rpc("/nonexistent-form-checkbox-rpc")
                    .with_initial(true),
                FormCheckbox::new("empty-checkbox", ""),
                FormCheckbox::new(
                    "mmmm-mmmm-checkbox",
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                ),
                FormCheckbox::new(
                    "mmmmmmmm-checkbox",
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                ),
            )),
        ),
    )
}

#[allow(clippy::too_many_lines)]
pub fn add_form_detail_page(keys: &mut KeySet<SessionState>) -> PageKey {
    let pressed = keys.add_static_page(
        "/form-detail-pressed",
        NavPage::new("Form Detail Pressed", Empty::new()),
    );
    // NOTE: If rust-fmt refuses to format this, try making all lines shorter, under the limit.
    keys.add_static_page(
        "/form-detail",
        NavPage::new(
            "Form Detail",
            Form::new((
                FormSection::new().with_title("Text").with_widgets((
                    FormDetail::new("Text").with_action(push(&pressed)),
                    FormDetail::new("Disabled"),
                    FormDetail::new("Does Nothing").with_action(nothing()),
                    FormDetail::new("").with_action(push(&pressed)),
                    FormDetail::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_action(push(&pressed)),
                    FormDetail::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_action(push(&pressed)),
                )),
                FormSection::new()
                    .with_title("Text + Sub-text")
                    .with_widgets((
                    FormDetail::new("Text")
                        .with_sub_text("Sub-text")
                        .with_action(push(&pressed)),
                    FormDetail::new("Disabled").with_sub_text("Sub-text"),
                    FormDetail::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_action(push(&pressed)),
                    FormDetail::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_action(push(&pressed)),
                    FormDetail::new("Text")
                        .with_sub_text(
                            "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                        )
                        .with_action(push(&pressed)),
                    FormDetail::new("Text")
                        .with_sub_text(
                            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                        )
                        .with_action(push(&pressed)),
                    FormDetail::new("")
                        .with_sub_text("")
                        .with_action(push(&pressed)),
                    FormDetail::new("Sub-text is empty")
                        .with_sub_text("")
                        .with_action(push(&pressed)),
                    FormDetail::new("")
                        .with_sub_text("Text is empty")
                        .with_action(push(&pressed)),
                )),
                FormSection::new().with_title("Image + Text").with_widgets((
                    FormDetail::new("Text")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("Disabled").with_photo_url("/placeholder-200x200.png"),
                    FormDetail::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    FormDetail::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    FormDetail::new("")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("Image not found")
                        .with_photo_url("/nonexistent")
                        .with_action(push(&pressed)),
                    FormDetail::new("Not an image")
                        .with_photo_url("/health")
                        .with_action(push(&pressed)),
                    // TODO: Use a URL that never returns a result.
                )),
                FormSection::new()
                    .with_title("Image + Text + Sub-text")
                    .with_widgets((
                    FormDetail::new("Text")
                        .with_sub_text("Sub-text")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("Disabled")
                        .with_sub_text("Sub-text")
                        .with_photo_url("/placeholder-200x200.png"),
                    FormDetail::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    FormDetail::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    FormDetail::new("Text")
                        .with_sub_text(
                            "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                        )
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("Text")
                        .with_sub_text(
                            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                        )
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("")
                        .with_sub_text("")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("Sub-text is empty")
                        .with_sub_text("")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    FormDetail::new("")
                        .with_sub_text("Text is empty")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                )),
            )),
        ),
    )
}

pub fn add_form_error_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-error",
        NavPage::new(
            "Form Error",
            Form::new((
                FormError::new("Error Message"),
                FormError::new(""),
                FormError::new(
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                ),
                FormError::new(
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                ),
            )),
        ),
    )
}

pub fn add_form_section_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-section",
        NavPage::new(
            "Form Section",
            Form::new((
                FormSection::new()
                    .with_title("Section A")
                    .with_widgets((Text::new("aaa"), Text::new("aaaa"))),
                FormSection::new().with_title("Empty Section"),
                FormSection::new()
                    .with_title("Section B")
                    .with_widgets((Text::new("bbb"), Text::new("bbbb"))),
                FormSection::new().with_widgets((
                    Text::new("First item of a section with no title."),
                    Text::new("Below is an empty section with no title."),
                )),
                FormSection::new(),
                FormSection::new()
                    .with_title("Section C")
                    .with_widgets((Text::new("ccc"), Text::new("cccc"))),
            )),
        ),
    )
}

pub fn add_form_text_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-text",
        NavPage::new(
            "Form Text",
            Form::new((
                Text::new("Text"),
                Text::new(""),
                Text::new("MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM"),
                Text::new("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"),
            )),
        ),
    )
}
