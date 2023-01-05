use crate::{SessionState, TEXTFIELD_CHECK_RPC_PATH};
use applin::action::{nothing, push};
use applin::session::{PageKey, PageMap};
use applin::widget::{
    AlertModal, Form, FormButton, FormSection, FormTextfield, NavPage, Scroll, Text,
};

pub fn add_form_button_page(keys: &mut PageMap<SessionState>) -> PageKey {
    let pressed = keys.add_static_page(
        "/form-button-pressed",
        AlertModal::new("Form Button Pressed").with_ok(),
    );
    keys.add_static_page(
        "/form-button",
        NavPage::new(
            "Form Button",
            Scroll::new(Form::new((
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
            ))),
        ),
    )
}

pub fn add_form_section_page(keys: &mut PageMap<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-section",
        NavPage::new(
            "Form Section",
            Scroll::new(Form::new((
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
            ))),
        ),
    )
}

pub fn add_form_text_field_page(keys: &mut PageMap<SessionState>) -> PageKey {
    keys.add_static_page(
        "/form-text-field",
        NavPage::new(
            "Form Text Field",
            Scroll::new(Form::new((
                FormTextfield::new("text1", "Enter some text"),
                FormTextfield::new("prefilled1", "Pre-filled").with_initial("initial content"),
                FormTextfield::new("rpc_checked1", "Checked via RPC (rejects the word 'bad')")
                    .with_check_rpc(TEXTFIELD_CHECK_RPC_PATH),
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
            ))),
        ),
    )
}
