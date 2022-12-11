use crate::{ServerState, SessionState, CHECK_VARS_RPC_PATH};
use applin::session::{KeySet, PageKey};
use applin::widget::{Checkbox, Form, NavPage};
use serde::{Deserialize, Serialize};
use servlin::{Request, Response};
use std::sync::Arc;

pub fn check_vars_rpc(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    #[derive(Default, Deserialize, Serialize)]
    struct Vars {
        // TODO: Find a way to catch typos in variable names.
        #[serde(skip_serializing_if = "Option::is_none")]
        option_a: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        option_b: Option<bool>,
    }
    let session = state.sessions.get(req)?;
    let input: Vars = req.json()?;
    let mut output = Vars::default();
    if !input.option_a.unwrap_or(false) {
        output.option_b = Some(false);
    }
    session.rpc_response_with_vars(output)
}

pub fn add_check_vars_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/check-vars",
        NavPage::new(
            "Check Vars",
            Form::new((
                Checkbox::new("option_a", "Option A").with_rpc(CHECK_VARS_RPC_PATH),
                Checkbox::new("option_b", "Option B (requires Option A)")
                    .with_rpc(CHECK_VARS_RPC_PATH),
            )),
        ),
    )
}
