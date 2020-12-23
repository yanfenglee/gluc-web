
use user_service::UserService;

use crate::config::CONFIG;

mod user_service;

lazy_static! {
   pub static ref CGM_SERVICE: UserService = UserService{};
}
