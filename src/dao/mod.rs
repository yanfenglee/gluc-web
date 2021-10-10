
use rbatis::rbatis::Rbatis;

pub mod mapper;

// 示例-Rbatis示例初始化(必须)
lazy_static! {
  pub static ref RB:Rbatis={
     let rbatis = Rbatis::new();
     return rbatis;
  };
}

///测试
#[cfg(test)]
mod test {
    //use rbatis::executor::Executor;
    //use serde_json::json;

    use crate::config::CONFIG;
    use crate::dao::RB;

    #[async_std::test]
    async fn test_rbatis() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None,true).unwrap();
        RB.link(&CONFIG.mysql_url).await.unwrap();
    }
}


