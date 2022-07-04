use crate::data::random_u64;
use crate::error::client_error;
use crate::session::SessionId;
use core::fmt::{Debug, Formatter};
use servlin::internal::escape_and_elide;
use servlin::{AsciiString, Cookie, Request, Response};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

const SESSION_COOKIE_NAME: &str = "session";

// TODONT: Do not implement `Ord` or `PartialOrd`.  They would let
//         data structure operations leak `secret` via timing.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct SessionCookie {
    id: u64,
    secret: u64,
}
impl SessionCookie {
    /// Returns `None` when the request doesn't have the cookie.
    ///
    /// # Errors
    /// Returns an error when the request has the cookie and we fail to parse it.
    pub fn from_req_option(req: &Request) -> Result<Option<SessionCookie>, Response> {
        if let Some(string) = req.cookies.get(SESSION_COOKIE_NAME) {
            let cookie = Self::try_from(string.as_str()).map_err(|e| {
                client_error(format!(
                    "error parsing {:?} cookie: {}",
                    SESSION_COOKIE_NAME, e
                ))
            })?;
            Ok(Some(cookie))
        } else {
            Ok(None)
        }
    }

    /// # Errors
    /// Returns an error when the request doesn't have the cookie or we fail to parse it.
    pub fn from_req(req: &Request) -> Result<SessionCookie, Response> {
        Self::from_req_option(req)?
            .ok_or_else(|| client_error(format!("missing cookie {:?}", SESSION_COOKIE_NAME)))
    }

    #[must_use]
    pub fn new_random() -> Self {
        Self {
            id: random_u64(),
            secret: random_u64(),
        }
    }

    #[must_use]
    pub fn id(&self) -> SessionId {
        SessionId::new(self.id)
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_cookie(&self) -> Cookie {
        Cookie::new(
            SESSION_COOKIE_NAME,
            AsciiString::try_from(format!("{}-{}", self.id, self.secret)).unwrap(),
        )
        .with_secure(false) // So we can test at http://127.0.0.1/.
    }
}
impl TryFrom<&str> for SessionCookie {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let err = || {
            format!(
                "invalid SessionCookie: {}",
                escape_and_elide(s.as_bytes(), 100)
            )
        };
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(err());
        }
        let id: u64 = parts[0].parse().map_err(|_| err())?;
        let secret: u64 = parts[1].parse().map_err(|_| err())?;
        Ok(Self { id, secret })
    }
}
impl FromStr for SessionCookie {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}
impl Debug for SessionCookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "SessionCookie(id={},secret=...)", self.id)
    }
}
#[allow(clippy::derive_hash_xor_eq)]
impl Hash for SessionCookie {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
        // TODONT: Do not hash `secret`.  This should prevent data structure operations
        // from leaking it via timing.
    }
}
