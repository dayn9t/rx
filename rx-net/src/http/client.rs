use reqwest::{Client, Error, header};
use rx_core::prelude::*;
use std::collections::HashMap;
use url::Url;

/// 数据访问对象客户端
///
/// `DaoListClient` 是一个通用的 RESTful API 客户端，专为与数据服务交互而设计。
/// 它封装了基本的 CRUD 操作（创建、读取、更新、删除），简化了与后端 API 的通信过程。
///
pub struct DaoListClient {
    // API 基础 URL，所有请求都基于此 URL 构建
    base_url: String,
    // reqwest HTTP 客户端实例，用于发送请求
    client: Client,
    // 请求头信息映射表，包含内容类型和可选的授权令牌
    headers: HashMap<String, String>,
}

impl DaoListClient {
    /// 创建一个新的 DaoListClient 实例
    ///
    /// # 参数
    ///
    /// * `base_url` - API 的基础 URL（例如 "http://api.example.com/v1"）
    ///   如果 URL 以斜杠结尾，该方法会自动移除末尾斜杠
    ///
    /// # 返回值
    ///
    /// 返回一个配置了基本请求头的新 `DaoListClient` 实例，默认设置 Content-Type 为 application/json
    ///
    pub fn new(base_url: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        DaoListClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
            headers,
        }
    }

    /// 将 HashMap 类型的请求头转换为 reqwest 的 HeaderMap 类型
    ///
    /// 该方法将内部存储的 headers HashMap 转换为 reqwest API 所需的 HeaderMap 格式。
    /// 如果某个头部名称或值无法转换（不符合 HTTP 标准），则该头部将被忽略。
    ///
    /// # 返回值
    ///
    /// 返回包含所有可以成功转换的头部的 `header::HeaderMap` 实例
    fn create_header_map(&self) -> header::HeaderMap {
        let mut header_map = header::HeaderMap::new();
        for (key, value) in &self.headers {
            if let (Ok(name), Ok(val)) = (
                header::HeaderName::from_bytes(key.as_bytes()),
                header::HeaderValue::from_str(value),
            ) {
                header_map.insert(name, val);
            }
        }
        header_map
    }

    /// 获取指定表中的所有记录
    ///
    /// # 类型参数
    ///
    /// * `T` - 响应数据的类型，必须实现 DeserializeOwned trait，以便从 JSON 响应解析
    ///
    /// # 参数
    ///
    /// * `table_name` - 数据表名称或集合名称
    /// * `params` - 可选的查询参数，作为 URL 查询字符串附加到请求
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Result<Vec<T>, Error>`，包含所有记录的向量
    /// 失败时返回 reqwest 错误
    ///
    pub async fn get_all<T: DeserializeOwned>(
        &self,
        table_name: &str,
        params: Option<&HashMap<String, String>>,
    ) -> Result<Vec<T>, Error> {
        let mut url = format!("{}/{}", self.base_url, table_name);

        // 如果有查询参数，手动构建带查询字符串的 URL
        if let Some(p) = params
            && let Ok(mut parsed_url) = Url::parse(&url)
        {
            for (key, value) in p {
                parsed_url.query_pairs_mut().append_pair(key, value);
            }
            url = parsed_url.to_string();
        }

        let resp = self
            .client
            .get(&url)
            .headers(self.create_header_map())
            .send()
            .await?
            .json::<Vec<T>>()
            .await?;
        Ok(resp)
    }

    /// 获取指定表中的单条记录
    ///
    /// # 类型参数
    ///
    /// * `T` - 响应数据的类型，必须实现 DeserializeOwned trait
    ///
    /// # 参数
    ///
    /// * `table_name` - 数据表名称或集合名称
    /// * `record_id` - 要获取的记录的唯一标识符
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Result<T, Error>`，包含请求的单个记录
    /// 失败时返回 reqwest 错误
    ///
    pub async fn get<T: DeserializeOwned>(
        &self,
        table_name: &str,
        record_id: &str,
    ) -> Result<T, Error> {
        let url = format!("{}/{}/{}", self.base_url, table_name, record_id);
        let resp = self
            .client
            .get(&url)
            .headers(self.create_header_map())
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(resp)
    }

    /// 创建新记录
    ///
    /// 向指定表中发送 POST 请求创建新记录。
    ///
    /// # 类型参数
    ///
    /// * `T` - 数据类型，必须同时实现 Serialize（用于发送数据）和 DeserializeOwned（用于接收响应）
    ///
    /// # 参数
    ///
    /// * `table_name` - 数据表名称或集合名称
    /// * `record` - 要创建的记录数据
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Result<T, Error>`，通常包含创建后的记录（可能包含服务器生成的 ID 或时间戳）
    /// 失败时返回 reqwest 错误
    ///
    pub async fn post<T: Serialize + DeserializeOwned>(
        &self,
        table_name: &str,
        record: &T,
    ) -> Result<T, Error> {
        let url = format!("{}/{}", self.base_url, table_name);
        let resp = self
            .client
            .post(&url)
            .headers(self.create_header_map())
            .json(record)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(resp)
    }

    /// 更新现有记录
    ///
    /// 通过 PUT 请求更新指定 ID 的记录。
    ///
    /// # 类型参数
    ///
    /// * `T` - 数据类型，必须同时实现 Serialize 和 DeserializeOwned
    ///
    /// # 参数
    ///
    /// * `table_name` - 数据表名称或集合名称
    /// * `record_id` - 要更新的记录的唯一标识符
    /// * `record` - 包含更新数据的记录对象
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Result<T, Error>`，包含更新后的记录
    /// 失败时返回 reqwest 错误
    ///
    pub async fn put<T: Serialize + DeserializeOwned>(
        &self,
        table_name: &str,
        record_id: &str,
        record: &T,
    ) -> Result<T, Error> {
        let url = format!("{}/{}/{}", self.base_url, table_name, record_id);
        let resp = self
            .client
            .put(&url)
            .headers(self.create_header_map())
            .json(record)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(resp)
    }

    /// 删除记录
    ///
    /// 通过 DELETE 请求删除指定 ID 的记录。
    ///
    /// # 参数
    ///
    /// * `table_name` - 数据表名称或集合名称
    /// * `record_id` - 要删除的记录的唯一标识符
    ///
    /// # 返回值
    ///
    /// 成功时返回 `Result<bool, Error>`，值为 true 表示删除成功
    /// 失败时返回 reqwest 错误
    ///
    pub async fn delete(&self, table_name: &str, record_id: &str) -> Result<bool, Error> {
        let url = format!("{}/{}/{}", self.base_url, table_name, record_id);
        self.client
            .delete(&url)
            .headers(self.create_header_map())
            .send()
            .await?
            .error_for_status()?; // 只检查状态码
        Ok(true)
    }

    /// 设置身份验证令牌
    ///
    /// 将 Bearer 令牌添加到请求头，用于后续所有请求的身份验证。
    ///
    /// # 参数
    ///
    /// * `token` - 认证令牌，将被自动添加 "Bearer " 前缀
    ///
    pub fn set_auth_token(&mut self, token: &str) {
        self.headers
            .insert("Authorization".to_string(), format!("Bearer {}", token));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
