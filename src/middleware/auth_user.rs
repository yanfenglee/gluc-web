use actix_web::{FromRequest, HttpRequest};
use futures::future::{Ready, ok, err};
use actix_http::{Payload, Error};
use actix_http::error::ParseError;
use actix_http::http::HeaderMap;

#[derive(Debug)]
pub struct AuthUser {
    pub(crate) user_id: i64,
    pub(crate) token: String,
}

impl AuthUser {
    pub fn from_header(headers: &HeaderMap) -> Option<Self> {
        if let Some(header) = headers.get("api_secret") {
            let token = header.to_str().unwrap();

            /// TODO async query db by token, and cache it
            Some(AuthUser { user_id: 42, token: token.into() })
        } else {
            None
        }
    }
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(
        req: &HttpRequest,
        _payload: &mut Payload,
    ) -> Self::Future {
        if let Some(user) = AuthUser::from_header(req.headers()) {
            ok(user)
        } else {
            err(Error::from(ParseError::Header))
        }
    }
}