use beatrice::Response;
use serde_json::json;

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::module_name_repetitions)]
pub fn client_error(message: impl ToString) -> Response {
    Response::text(400, message.to_string())
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::module_name_repetitions)]
pub fn server_error(message: impl ToString) -> Response {
    Response::text(500, message.to_string())
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::module_name_repetitions)]
pub fn user_error(message: impl AsRef<str>) -> Response {
    Response::json(400, json!({"message": message.as_ref()})).unwrap()
}
