use app_config::ApplicationConfig;

pub mod app_config;
pub mod log_config;

//当前服务配置
lazy_static! {
      pub static ref CONFIG:ApplicationConfig=ApplicationConfig::default();
}