use dirs::home_dir;
use rx_core::log::*;
use rx_db::path;
use rx_rest::api::{ApiCfg, run_api};
use rx_rest::task::TaskApiService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志，设置日志级别为调试级别
    init_log(2); // 使用TRACE级别记录日志

    let home = home_dir().expect("无法获取用户主目录");
    let db_root = path!(home / "tmp/api-t2");
    let ias_api = TaskApiService::new(&db_root, "task", "status");

    //println!("full_name: {}", app_params.info.full_name());
    let cfg = ApiCfg::default();
    let version = "v1.0.0";
    run_api(ias_api, cfg, version).await
}
