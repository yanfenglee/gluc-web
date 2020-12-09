
use cgm_service::CgmService;

use crate::config::CONFIG;

mod cgm_service;

lazy_static! {
   pub static ref CGM_SERVICE: CgmService = CgmService{};
}
