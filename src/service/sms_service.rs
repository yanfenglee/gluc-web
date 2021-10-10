
use crate::domain::dto::{VerifyCodeDTO};


use crate::base::resp::{Resp};
///User service
pub struct SmsService {}

const SEND_INTERVAL: i64 = 60000;

impl SmsService {
    pub async fn send_code(&self, arg: &VerifyCodeDTO) -> Resp<i64> {
        // send sms code

        log::info!("register info: {:?}", arg);

        let res = Resp {
            code: "0".to_string(),
            msg: None,
            data: Some(SEND_INTERVAL)
        };

        res
    }

}
