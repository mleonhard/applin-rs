use crate::data::random_positive_nonzero_i64;
use crate::error::client_error;
use crate::session::{ServerInstanceId, SessionId};
use core::fmt::{Debug, Formatter};
use servlin::internal::escape_and_elide;
use servlin::{AsciiString, Cookie, Request, Response};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

const SYNC_COOKIE_NAME: &str = "APPLIN_SYNC";

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SyncCookie {
    pub id: ServerInstanceId,
    pub sequence: i64,
}
impl SyncCookie {
    /// Returns `None` when the request doesn't have the cookie.
    ///
    /// # Errors
    /// Returns an error when the request has the cookie and we fail to parse it.
    pub fn from_req_option(req: &Request) -> Result<Option<SyncCookie>, Response> {
        if let Some(string) = req.cookies.get(SYNC_COOKIE_NAME) {
            let cookie = Self::try_from(string.as_str()).map_err(|e| {
                client_error(format!("error parsing {SYNC_COOKIE_NAME:?} cookie: {e}"))
            })?;
            Ok(Some(cookie))
        } else {
            Ok(None)
        }
    }

    /// # Errors
    /// Returns an error when the request doesn't have the cookie or we fail to parse it.
    pub fn from_req(req: &Request) -> Result<SyncCookie, Response> {
        Self::from_req_option(req)?
            .ok_or_else(|| client_error(format!("missing cookie {SYNC_COOKIE_NAME:?}")))
    }

    #[must_use]
    pub fn new_zero(id: ServerInstanceId) -> Self {
        Self { id, sequence: 0 }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn to_cookie(&self) -> Cookie {
        Cookie::new(
            SYNC_COOKIE_NAME,
            AsciiString::try_from(format!("{}-{}", self.id.0, self.sequence)).unwrap(),
        )
        .with_secure(false) // So we can test at http://127.0.0.1/.
    }
}
impl TryFrom<&str> for SyncCookie {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let err = || {
            format!(
                "invalid SyncCookie: {}",
                escape_and_elide(s.as_bytes(), 100)
            )
        };
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(err());
        }
        let id: i64 = parts[0].parse().map_err(|_| err())?;
        if id < 0 {
            return Err(err());
        }
        let sequence: i64 = parts[1].parse().map_err(|_| err())?;
        if sequence < 0 {
            return Err(err());
        }
        Ok(Self {
            id: ServerInstanceId(id),
            sequence,
        })
    }
}
impl FromStr for SyncCookie {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }
}
impl Debug for SyncCookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "SyncCookie(id={},sequence={})", self.id, self.sequence)
    }
}
