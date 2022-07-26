use crate::{ServerState, SessionState};
use applin::session::{KeySet, PageKey};
use applin::widget::{Form, FormCheckbox, NavPage};
use serde::{Deserialize, Serialize};
use servlin::{Request, Response};
use std::sync::Arc;

pub static CHECK_VARS_RPC_PATH: &str = "/vars/check-vars-rpc";
pub static OPTION_A_NAME: &str = "vars_option_a";
pub static OPTION_B_NAME: &str = "vars_option_b";

pub fn check_vars_rpc(state: &Arc<ServerState>, req: &Request) -> Result<Response, Response> {
    #[derive(Default, Deserialize, Serialize)]
    struct Vars {
        // TODO: Find a way to catch typos in variable names.
        #[serde(skip_serializing_if = "Option::is_none")]
        vars_option_a: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        vars_option_b: Option<bool>,
    }
    let session = state.sessions.get(req)?;
    let input: Vars = req.json()?;
    let mut output = Vars::default();
    if input.vars_option_b.unwrap_or(false) && !input.vars_option_a.unwrap_or(false) {
        output.vars_option_a = Some(true);
    }
    session.rpc_response_with_vars(output)
}

pub fn add_check_vars_page(keys: &mut KeySet<SessionState>) -> PageKey {
    keys.add_static_page(
        "/vars/check-vars",
        NavPage::new(
            "Check Vars",
            Form::new((
                FormCheckbox::new(OPTION_A_NAME, "Option A").with_rpc(CHECK_VARS_RPC_PATH),
                FormCheckbox::new(OPTION_B_NAME, "Option B (requires Option A)")
                    .with_rpc(CHECK_VARS_RPC_PATH),
            )),
        ),
    )
}
