use crate::action::Action;
use crate::page_key::PageKey;

#[must_use]
pub fn copy_to_clipboard(s: impl Into<String>) -> Action {
    Action::CopyToClipboard(s.into())
}

#[must_use]
pub fn launch_url(s: impl Into<String>) -> Action {
    Action::LaunchUrl(s.into())
}

#[must_use]
pub fn push(page_key: &PageKey) -> Action {
    Action::Push(page_key.clone().into_inner())
}

#[must_use]
pub fn logout() -> Action {
    Action::Logout
}

#[must_use]
pub fn nothing() -> Action {
    Action::Nothing
}

#[must_use]
pub fn pop() -> Action {
    Action::Pop
}

#[must_use]
pub fn rpc(url: impl Into<String>) -> Action {
    Action::Rpc(url.into())
}
