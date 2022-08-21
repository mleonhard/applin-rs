use crate::{ServerState, SessionState};
use applin::action::{nothing, push};
use applin::error::user_error;
use applin::session::{KeySet, PageKey};
use applin::widget::{
    AlertModal, Empty, Form, FormButton, FormCheckbox, FormDetail, FormError, FormSection,
    FormTextfield, NavPage, Text,
};
use serde::Deserialize;
use servlin::{Request, Response};
use std::sync::Arc;

pub static FORM_CHECKBOX_RPC_PATH: &str = "/widgets/checkbox";

pub fn form_checkbox_rpc(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    let session = state.sessions.get(req)?;
    session.rpc_response()
}

pub static FORM_TEXTFIELD_CHECK_RPC_PATH: &str = "/widgets/form-textfield-check";

pub fn form_textfield_check_rpc(
    state: &Arc<ServerState>,
    req: &Request,
) -> Result<Response, Response> {
    #[derive(Deserialize)]
    struct Vars {
        rpc_checked1: String,
    }
    let _session = state.sessions.get(req)?;
    let vars: Vars = req.json()?;
    if vars.rpc_checked1.contains("bad") {
        Err(user_error("Please remove 'bad' from the box."))
    } else {
        Ok(Response::new(200))
    }
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

pub fn add_form_text_field_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-text-field",
        NavPage::new(
            "Form Text Field",
            Form::new((
                FormTextfield::new("text1", "Enter some text"),
                FormTextfield::new("prefilled1", "Pre-filled").with_initial("initial content"),
                FormTextfield::new("rpc_checked1", "Checked via RPC (rejects the word 'bad')")
                    .with_check_rpc(FORM_TEXTFIELD_CHECK_RPC_PATH),
                FormTextfield::new("nums1", "Numbers only").with_allow_numbers(),
                FormTextfield::new("ascii1", "ASCII only").with_allow_ascii(),
                FormTextfield::new("tel1", "Tel").with_allow_tel(),
                FormTextfield::new("email1", "Email").with_allow_email(),
                FormTextfield::new("names1", "Auto-capitalize for names").with_autocap_names(),
                FormTextfield::new("sentences1", "Auto-capitalize for sentences")
                    .with_autocap_sentences(),
                FormTextfield::new("minchars1", "3 chars required").with_min_chars(3),
                FormTextfield::new("maxchars1", "5 chars max").with_max_chars(5),
                FormTextfield::new("maxlines1", "Single-line").with_max_lines(1),
                FormTextfield::new("maxlines2", "Three lines max").with_max_lines(3),
                FormTextfield::new("mmms", "Pre-filled with many words").with_initial(
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                ),
                FormTextfield::new("mmmmm", "Pre-filled with a long word").with_initial(
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                ),
            )),
        ),
    )
}
