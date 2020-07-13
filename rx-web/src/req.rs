use std::collections::HashMap;

use http_req::uri::Uri;
use serde_derive::{Deserialize, Serialize};

pub use http_req::error::Error;
pub use http_req::request::Request;
pub use http_req::response::Response;

/// 请求配置信息
#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequestCfg {
    pub headers: HashMap<String, String>,
}

pub fn get(url: &str, writer: &mut Vec<u8>, cfg: &RequestCfg) -> Result<Response, Error> {
    let uri = url.parse::<Uri>().unwrap();
    let mut req = Request::new(&uri);

    for (k, v) in &cfg.headers {
        req.header(k, v);
    }
    req.send(writer)
}
