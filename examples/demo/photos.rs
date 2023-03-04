use crate::{ServerState, Session, GET_PHOTO_PATH};
use applin::session::{PageKey, PageMap};
use applin::widget::{Image, NavPage, Text};
use servlin::{ContentType, Request, Response};
use std::io::Read;
use std::sync::{Arc, PoisonError};

pub fn upload_photo_handler(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    if req.method != "POST" {
        return Err(Response::method_not_allowed_405(&["POST"]));
    }
    if req.content_type != ContentType::Jpeg {
        return Err(Response::text(415, "Only JPEG is accepted."));
    }
    let applin_session = state.sessions.get(req)?;
    if req.body.is_pending() {
        return Ok(Response::get_body_and_reprocess(10 * 1024 * 1024));
    }
    let mut body_bytes = Vec::new();
    let mut reader = req.body.reader()?;
    reader.read_to_end(&mut body_bytes)?;
    {
        let session_guard = applin_session
            .value
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let mut photo_guard = session_guard.photo.write(applin_session.rpc_context());
        *photo_guard = Some(body_bytes);
    }
    applin_session.rpc_response()
}

pub fn get_photo_handler(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    if req.method != "GET" {
        return Err(Response::method_not_allowed_405(&["GET"]));
    }
    let applin_session = state.sessions.get(req)?;
    let session_guard = applin_session
        .value
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    let photo_guard = session_guard.photo.write(applin_session.rpc_context());
    match &*photo_guard {
        None => Err(Response::not_found_404()),
        Some(bytes) => Ok(Response::new(200).with_body(bytes.clone())),
    }
}

pub fn add_view_photo_page(keys: &mut PageMap<Session>) -> PageKey {
    keys.add_page_fn("/get-photo", move |rebuilder| {
        let applin_session = rebuilder.session()?;
        let session_guard = applin_session
            .value
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let photo_guard = session_guard.photo.read(rebuilder);
        Ok(NavPage::new(
            "Photo",
            match *photo_guard {
                None => Text::new("No photo found.").to_widget(),
                Some(..) => Image::new(1.0, GET_PHOTO_PATH).to_widget(),
            },
        )
        .with_stream())
    })
}
