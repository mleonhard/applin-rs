use beatrice::Response;
use serde_json::json;

pub fn client_error(message: impl ToString) -> Response {
    Response::text(400, message.to_string())
}

pub fn server_error(message: impl ToString) -> Response {
    Response::text(500, message.to_string())
}

pub fn user_error(message: impl AsRef<str>) -> Response {
    Response::json(400, json!({"message": message.as_ref()})).unwrap()
}
