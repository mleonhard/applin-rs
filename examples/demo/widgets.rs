use crate::{Session, ERROR_RPC_PATH, OK_RPC_PATH};
use applin::action::{nothing, pop, push, rpc};
use applin::internal::ImageDisposition;
use applin::session::{PageKey, PageMap};
use applin::widget::{
    AlertModal, BackButton, Button, Checkbox, Column, Empty, ErrorText, Form, FormSection, Image,
    NavButton, NavPage, Scroll, Text,
};

pub fn add_back_button_pages(keys: &mut PageMap<Session>) -> PageKey {
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
                .with_action(rpc(OK_RPC_PATH))
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
                .with_action(rpc(ERROR_RPC_PATH))
                .with_action(pop()),
        ),
    );
    keys.add_static_page(
        "/back-button",
        NavPage::new(
            "Back Button",
            Scroll::new(Form::new((
                NavButton::new("Default").with_action(push(&default)),
                NavButton::new("Disabled").with_action(push(&disabled)),
                NavButton::new("Missing").with_action(push(&missing)),
                NavButton::new("RPC").with_action(push(&rpc_ok)),
                NavButton::new("RPC Error").with_action(push(&rpc_err)),
            ))),
        ),
    )
}

pub fn add_button_page(keys: &mut PageMap<Session>) -> PageKey {
    let pressed = keys.add_static_page(
        "/button-pressed",
        AlertModal::new("Button Pressed").with_ok(),
    );
    keys.add_static_page(
        "/button",
        NavPage::new(
            "Button",
            Scroll::new(Form::new((
                Button::new("Button").with_action(push(&pressed)),
                Button::new(
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                )
                .with_action(push(&pressed)),
                Button::new(
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                )
                .with_action(push(&pressed)),
                Button::new("").with_action(push(&pressed)),
                Button::new("Disabled Button"),
                Button::new("Does Nothing").with_action(nothing()),
            ))),
        ),
    )
}

pub fn add_checkbox_page(keys: &mut PageMap<Session>) -> PageKey {
    keys.add_static_page(
        "/checkbox",
        NavPage::new(
            "Checkbox",
            Scroll::new(Form::new((
                Checkbox::new("checkbox", "Checkbox"),
                Checkbox::new("initial-checked", "Initially checked").with_initial(true),
                Checkbox::new("with-rpc", "Does RPC on change").with_rpc(OK_RPC_PATH),
                Checkbox::new("with-bad-rpc", "Does RPC on change, but it fails")
                    .with_rpc(ERROR_RPC_PATH)
                    .with_initial(true),
                Checkbox::new("empty-checkbox", ""),
                Checkbox::new(
                    "mmmm-mmmm-checkbox",
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                ),
                Checkbox::new(
                    "mmmmmmmm-checkbox",
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                ),
            ))),
        ),
    )
}

pub fn add_error_text_page(keys: &mut PageMap<Session>) -> PageKey {
    keys.add_static_page(
        "/error-text",
        NavPage::new(
            "ErrorText",
            Scroll::new(Form::new((
                ErrorText::new("Error Message"),
                ErrorText::new(""),
                ErrorText::new(
                    "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                ),
                ErrorText::new(
                    "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                ),
            ))),
        ),
    )
}

pub fn add_image_page(keys: &mut PageMap<Session>) -> PageKey {
    keys.add_static_page(
        "/image",
        NavPage::new(
            "Image",
            Scroll::new(Form::new((
                Text::new("Fit"),
                Image::new(2.0, "/placeholder-200x200.png"),
                Text::new("Cover"),
                Image::new(2.0, "/placeholder-200x200.png")
                    .with_disposition(ImageDisposition::Cover),
                Text::new("Stretch"),
                Image::new(2.0, "/placeholder-200x200.png")
                    .with_disposition(ImageDisposition::Stretch),
                Text::new("Not found"),
                Image::new(4.0, "/nonexistent.png"),
            ))),
        ),
    )
}

#[allow(clippy::too_many_lines)]
pub fn add_nav_button_page(keys: &mut PageMap<Session>) -> PageKey {
    let pressed = keys.add_static_page(
        "/nav-button-pressed",
        NavPage::new("Nav Button Pressed", Empty::new()),
    );
    // NOTE: If rust-fmt refuses to format this, try making all lines shorter, under the limit.
    keys.add_static_page(
        "/nav-button",
        NavPage::new(
            "Nav Button",
            Scroll::new(Form::new((
                FormSection::new().with_title("Text").with_widgets((
                    NavButton::new("Text").with_action(push(&pressed)),
                    NavButton::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_action(push(&pressed)),
                    NavButton::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_action(push(&pressed)),
                    NavButton::new("").with_action(push(&pressed)),
                    NavButton::new("Disabled"),
                    NavButton::new("Does Nothing").with_action(nothing()),
                )),
                FormSection::new()
                    .with_title("Text + Sub-text")
                    .with_widgets((
                    NavButton::new("Text")
                        .with_sub_text("Sub-text")
                        .with_action(push(&pressed)),
                    NavButton::new("Disabled").with_sub_text("Sub-text"),
                    NavButton::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_action(push(&pressed)),
                    NavButton::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_action(push(&pressed)),
                    NavButton::new("Text")
                        .with_sub_text(
                            "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                        )
                        .with_action(push(&pressed)),
                    NavButton::new("Text")
                        .with_sub_text(
                            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                        )
                        .with_action(push(&pressed)),
                    NavButton::new("")
                        .with_sub_text("")
                        .with_action(push(&pressed)),
                    NavButton::new("Sub-text is empty")
                        .with_sub_text("")
                        .with_action(push(&pressed)),
                    NavButton::new("")
                        .with_sub_text("Text is empty")
                        .with_action(push(&pressed)),
                )),
                FormSection::new().with_title("Image + Text").with_widgets((
                    NavButton::new("Text")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    NavButton::new("Disabled").with_photo_url("/placeholder-200x200.png"),
                    NavButton::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    NavButton::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    NavButton::new("")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    NavButton::new("Image not found")
                        .with_photo_url("/nonexistent")
                        .with_action(push(&pressed)),
                    NavButton::new("Not an image")
                        .with_photo_url("/health")
                        .with_action(push(&pressed)),
                    // TODO: Use a URL that never returns a result.
                )),
                FormSection::new()
                    .with_title("Image + Text + Sub-text")
                    .with_widgets((
                    NavButton::new("Text")
                        .with_sub_text("Sub-text")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    NavButton::new("Disabled")
                        .with_sub_text("Sub-text")
                        .with_photo_url("/placeholder-200x200.png"),
                    NavButton::new(
                        "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    NavButton::new(
                        "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                    )
                    .with_sub_text("Sub-text")
                    .with_photo_url("/placeholder-200x200.png")
                    .with_action(push(&pressed)),
                    NavButton::new("Text")
                        .with_sub_text(
                            "MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM",
                        )
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    NavButton::new("Text")
                        .with_sub_text(
                            "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM",
                        )
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    NavButton::new("")
                        .with_sub_text("")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    NavButton::new("Sub-text is empty")
                        .with_sub_text("")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                    NavButton::new("")
                        .with_sub_text("Text is empty")
                        .with_photo_url("/placeholder-200x200.png")
                        .with_action(push(&pressed)),
                )),
            ))),
        ),
    )
}

pub fn add_text_page(keys: &mut PageMap<Session>) -> PageKey {
    keys.add_static_page(
        "/text",
        NavPage::new(
            "Text",
            Scroll::new(Form::new((
                Text::new("Text"),
                Text::new(""),
                Text::new("MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM MMMM"),
                Text::new("MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM"),
            ))),
        ),
    )
}
