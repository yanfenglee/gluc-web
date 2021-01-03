use actix_http::Response;
use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use rbatis::crud::CRUDEnable;
use rbatis::core::Error;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::base::resp::RespErr::SimpleError;
use sailfish::TemplateOnce;
use actix_http::http::StatusCode;
use actix_http::error::InternalError;

/// response struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resp<T> {
    pub code: String,
    pub msg: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RespErr {
    SimpleError(String),
    CodeError(String, String),
}

impl From<Error> for RespErr {
    fn from(e: Error) -> Self {
        SimpleError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, RespErr>;


/// Returns Resp<T> json.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use gluc_web::base::resp::resp;
/// use gluc_web::base::resp::RespErr::SimpleError;
/// use gluc_web::base::resp::RespErr::CodeError;
///
/// #[derive(Serialize, Deserialize, Clone, Debug)]
/// pub struct UserDTO {
///     pub username: String,
///     pub password: String,
/// }
///
/// let data = UserDTO {
///     username: "xiaoming".into(),
///     password: "123456".into,
/// };
///
/// resp(&Ok(data));
/// resp(&Err(SimpleError("服务器内部异常".into())));
/// resp(&Err(CodeError("1001".into(), "账号未登录".into())));
///
/// ```
pub fn resp<T>(arg: &Result<T>) -> Response where T: Serialize + DeserializeOwned + Clone {
    let res = match arg {
        Ok(data) => Resp { code: "0".into(), msg: None, data: Some(data) },
        Err(e) => {
            match e {
                RespErr::SimpleError(e) => Resp { code: "1111".into(), msg: Some(e.clone()), data: None },
                RespErr::CodeError(code, e) => Resp { code: code.clone(), msg: Some(e.clone()), data: None },
            }
        }
    };

    let json_str = serde_json::to_string(&res).unwrap();

    HttpResponse::Ok().content_type("json").body(json_str)
}

pub fn resp_html<T>(arg: T) -> Response where T: TemplateOnce {
    let ref res = match arg.render_once() {
        Ok(body) => body,
        Err(e) => e.to_string(),
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(res)
}

#[cfg(test)]
mod test {
    use crate::view::index::Index;
    use crate::base::resp::resp_html;

    #[test]
    fn test_html() {
        let val = "4.5".to_string();
        let delta = "0.3".to_string();
        let direction = "flat".to_string();

        let index = Index {
            val,
            delta,
            direction,
        };

        println!("resp html {:?}", resp_html(index));
    }
}