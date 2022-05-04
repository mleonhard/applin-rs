use beatrice::Response;
use serde_json::json;

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::module_name_repetitions)]
pub fn client_error(message: impl Into<String>) -> Response {
    Response::text(400, message.into())
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::module_name_repetitions)]
pub fn server_error(message: impl Into<String>) -> Response {
    Response::text(500, message.into())
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::module_name_repetitions)]
pub fn user_error(message: impl AsRef<str>) -> Response {
    Response::json(400, json!({"message": message.as_ref()})).unwrap()
}
