use std::time::Duration;

use fast_log::consts::LogSize::MB;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::ZipPacker;

use crate::config::CONFIG;

pub fn init_log() {
    fast_log::init_split_log(
        "target/logs/",
        1000,
        MB(1),
        RollingType::KeepTime(Duration::from_secs(3600 * 24)),
        log::Level::Info,
        None,
        Box::new(ZipPacker {}),
        CONFIG.debug,
    );
}
