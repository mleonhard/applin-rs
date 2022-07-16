use crate::session::SessionId;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Context {
    Rpc(SessionId),
    Empty,
}
