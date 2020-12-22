use actix_web::{FromRequest, HttpRequest};
use futures::future::{Ready, ok, err};
use actix_http::{Payload, Error};
use actix_http::error::ParseError;
use actix_http::http::HeaderMap;
use std::pin::Pin;
use futures::{Future, FutureExt};

use crate::dao::RB;

#[derive(Debug)]
pub struct AuthUser {
    pub(crate) user_id: i64,
    pub(crate) token: String,
}

impl AuthUser {
    pub async fn from_header(headers: &HeaderMap) -> Option<Self> {
        if let Some(header) = headers.get("api_secret") {
            let token = header.to_str().unwrap();

            #[py_sql(RB, "SELECT id FROM users WHERE token = #{token} LIMIT 1")]
            fn select_id(token: &str) -> Option<i64> {}

            if let Ok(Some(id)) = select_id(token).await {
                return Some(AuthUser { user_id: id, token: token.into() });
            }

        }

        None
    }
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(
        req: &HttpRequest,
        _payload: &mut Payload,
    ) -> Self::Future {
        let headers = req.headers().clone();

        let ret = async move {
            if let Some(user) = AuthUser::from_header(&headers).await {
                Ok(user)
            } else {
                Err(Error::from(ParseError::Header))
            }
        };

        Box::pin(ret)
    }
}