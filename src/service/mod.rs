use user_service::UserService;
use sms_service::SmsService;

mod user_service;
mod sms_service;

lazy_static! {
    pub static ref USER_SERVICE: UserService = UserService{};
    pub static ref SMS_SERVICE: SmsService = SmsService{};
}
