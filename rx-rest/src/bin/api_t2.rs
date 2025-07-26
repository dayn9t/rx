use rx_db::path;
use rx_rest::api::{ApiCfg, run_api};
use rx_rest::task::ApiService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_root = path!("/tmp/api-t2");
    let ias_api = ApiService::new(&db_root, "task", "status");

    //println!("full_name: {}", app_params.info.full_name());
    let cfg = ApiCfg::default();
    let version = "v1.0.0";
    println!("cfg: {:?}", cfg);
    run_api(ias_api, cfg, version).await
}
