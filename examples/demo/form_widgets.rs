use crate::Session;
use applin::action::{nothing, push};
use applin::session::{PageKey, PageMap};
use applin::widget::{AlertModal, Form, FormButton, FormSection, NavPage, Scroll, Text};

pub fn add_form_button_page(keys: &mut PageMap<Session>) -> PageKey {
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

pub fn add_form_section_page(keys: &mut PageMap<Session>) -> PageKey {
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
