use poem::{EndpointExt, Result, Route, Server, listener::TcpListener, middleware::Cors};
use poem_openapi::{OpenApi, OpenApiService};
use rx_core::log::*;
use rx_core::prelude::*;
use rx_net::cfg::get_adapters_ip4s;
use std::fmt::Display;

use crate::api::common::Object;
use std::process;
use tokio::time::sleep;

const ALL_ADDR: &'static str = "0.0.0.0";
const LOCAL_ADDR: &'static str = "127.0.0.1";

/// Endpoint信息
#[derive(Object, Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// 主机
    pub host: String,
    /// 端口
    pub port: u16,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            host: ALL_ADDR.to_string(),
            port: 8080,
        }
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}:{}", &self.host, &self.port))
    }
}

/// 授权信息
#[derive(Object, Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    /// 用户
    pub user: String,
    /// 密码
    pub password: String,
}

/// Api程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCfg {
    /// 服务标题
    pub title: String,
    /// URL root
    pub root: String,
    /// 服务监听的Endpoint
    pub endpoint: Endpoint,
}
impl Default for ApiCfg {
    fn default() -> Self {
        Self {
            title: "RxApi".to_string(),
            root: "api".to_string(),
            endpoint: Endpoint::default(),
        }
    }
}

impl ApiCfg {
    pub fn url(&self, scheme: &str) -> String {
        format!("{}://{}/{}", scheme, self.endpoint.to_string(), &self.root)
    }

    pub fn http_url(&self) -> String {
        self.url("http")
    }
}

pub async fn run_api<T: OpenApi + 'static>(
    ias_api: T,
    cfg: ApiCfg,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let ip4s = if cfg.endpoint.host == ALL_ADDR {
        get_adapters_ip4s()
            .iter()
            .map(|addr| addr.ip.to_string())
            .collect()
    } else {
        vec![cfg.endpoint.host.clone()]
    };

    let mut server_count = 0;
    let mut api_service = OpenApiService::new(ias_api, cfg.title, version);
    for ip in ip4s {
        if ip != LOCAL_ADDR {
            let service_url = format!("http://{}:{}{}", ip, cfg.endpoint.port, cfg.root);
            info!("Swagger: {}/ui", &service_url);
            api_service = api_service.server(service_url);
            server_count += 1;
        }
    }

    if server_count < 1 {
        eprintln!("No valid ip4 address found!");
        sleep(std::time::Duration::from_secs(2)).await;
        process::exit(1);
    }

    let redoc = api_service.redoc();
    let rapidoc = api_service.rapidoc();

    let swagger = api_service.swagger_ui();
    let spec = api_service.spec();
    let route = Route::new()
        .nest(&cfg.root, api_service)
        .nest(cfg.root.clone() + "/swagger", swagger)
        .nest(cfg.root.clone() + "/redoc", redoc)
        .nest(cfg.root.clone() + "/rapidoc", rapidoc)
        .at(
            cfg.root + "/spec",
            poem::endpoint::make_sync(move |_| spec.clone()),
        )
        .with(Cors::new());

    Server::new(TcpListener::bind(cfg.endpoint.to_string()))
        .run(route)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_task() {}
}
