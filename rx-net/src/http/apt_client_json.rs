use reqwest::blocking;
use reqwest::header::ACCEPT;
use serde::de::DeserializeOwned;

use rx_core::text::BoxResult;

use crate::url::split_url2;

/// HTTP API Client 使用 JSON 格式
pub struct ApiClientJson {
    url_base: String,
}

impl ApiClientJson {
    pub fn new(url_base: &str) -> Self {
        Self {
            url_base: url_base.to_string(),
        }
    }

    /// 从指定的 URL 获取数据(同步), 并转反序列化为指定类型
    pub fn get<T: DeserializeOwned>(&self, path: impl AsRef<str>) -> BoxResult<T> {
        let path = path.as_ref();
        let client = blocking::Client::new();
        let response = client
            .get(&format!("{}{}", self.url_base, path))
            .header(ACCEPT, "text/json")
            .send()?
            .text()?;
        let data: T = serde_json::from_str(&response)?;
        Ok(data)
    }
}

/// 从指定的 URL 获取数据(同步), 并转反序列化为指定类型
pub fn get<T: DeserializeOwned>(url: impl AsRef<str>) -> BoxResult<T> {
    let (url_base, path) = split_url2(url)?;

    let client = ApiClientJson::new(&url_base);
    let data: T = client.get(path)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
