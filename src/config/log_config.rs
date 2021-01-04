use fast_log::appender::{LogAppender, FastLogFormatRecord};
use fast_log::filter::ModuleFilter;
use fast_log::plugin::console::ConsoleAppender;
use fast_log::plugin::file_split::{FileSplitAppender, RollingType};
use fast_log::consts::LogSize::KB;

use crate::config::CONFIG;

pub fn init_log() {
    //自定义日志追加器
    let mut appenders: Vec<Box<dyn LogAppender>> = vec![
        Box::new(FileSplitAppender::new("target/logs/",
                                        KB(128),
                                        RollingType::KeepNum(5),
                                        true,
                                        10))
    ];
    if CONFIG.debug {
        appenders.push(Box::new(ConsoleAppender {}));
    }
    //自定义日志过滤
    fast_log::init_custom_log(appenders,
                              1000,
                              if CONFIG.debug {log::Level::Debug} else {log::Level::Info},
                              Box::new(ModuleFilter::new_exclude(vec!["sqlx".to_string()])),
                              Box::new(FastLogFormatRecord{})
    );
}


