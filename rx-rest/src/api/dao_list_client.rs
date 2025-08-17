use anyhow::{Result, anyhow};
use poem::http::Method;
use reqwest::{Client, IntoUrl, Response, header};
use rx_core::prelude::*;
use rx_db::IRecord;
use std::collections::HashMap;
use tokio::sync::Mutex;

/// API客户端返回结果类型
pub type ResultE<T> = Result<T>;

pub type ParamsMap = HashMap<String, String>;

/// DAO列表客户端
///
/// 提供对REST API的基础操作
pub struct DaoListClient {
    /// API基础URL
    base_url: String,
    /// HTTP客户端
    client: Mutex<Client>,
    /// 认证令牌
    auth_token: Mutex<Option<String>>,
}

impl DaoListClient {
    /// 创建新的DAO列表客户端
    ///
    /// # 参数
    /// * `base_url` - API基础URL
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: Mutex::new(Client::new()),
            auth_token: Mutex::new(None),
        }
    }

    /// 设置认证令牌
    ///
    /// # 参数
    /// * `token` - 认证令牌
    pub async fn set_auth_token(&self, token: impl Into<String>) {
        let mut auth_token = self.auth_token.lock().await;
        *auth_token = Some(token.into());
    }

    /// 构建请求URL
    ///
    /// # 参数
    /// * `table_name` - 表名
    /// * `id` - 可选的记录ID
    fn build_url(&self, table_name: &str, id: Option<&str>) -> String {
        match id {
            Some(id) => format!("{}/{}/{}", self.base_url, table_name, id),
            None => format!("{}/{}", self.base_url, table_name),
        }
    }

    /// 准备请求：应用认证信息并添加查询参数
    ///
    /// # 参数
    /// * `url` - 请求URL
    /// * `method` - HTTP方法
    /// * `params` - 可选的查询参数
    ///
    /// # 返回
    /// * 添加了认证和查询参数的请求构建器
    async fn prepare_request(
        &self,
        url: impl IntoUrl,
        method: Method,
        params: Option<&ParamsMap>,
    ) -> reqwest::RequestBuilder {
        let client = self.client.lock().await;
        let builder = client.request(method, url);

        let auth_token = self.auth_token.lock().await;
        let builder = match &*auth_token {
            Some(token) => builder.header(header::AUTHORIZATION, format!("Bearer {}", token)),
            None => builder,
        };

        if let Some(p) = params {
            builder.query(p)
        } else {
            builder
        }
    }

    /// 执行请求并处理响应
    ///
    /// # 参数
    /// * `builder` - 请求构建器
    ///
    /// # 返回
    /// * `ResultE<Response>` - HTTP响应或错误
    async fn execute_request(&self, builder: reqwest::RequestBuilder) -> ResultE<Response> {
        let response = builder.send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("API错误: {}", response.status()));
        }

        Ok(response)
    }

    /// 发送请求并反序列化响应为特定类型
    ///
    /// # 参数
    /// * `table_name` - 表名
    /// * `id` - 可选的记录ID
    /// * `method` - HTTP方法
    /// * `body` - 可选的请求体
    /// * `params` - 可选的查询参数
    ///
    /// # 返回
    /// * `ResultE<T>` - 反序列化的结果或错误
    async fn request<T>(
        &self,
        table_name: &str,
        id: Option<&str>,
        method: Method,
        body: Option<T>,
        params: Option<&ParamsMap>,
    ) -> ResultE<T>
    where
        T: DeserializeOwned + Serialize,
    {
        let url = self.build_url(table_name, id);
        let mut builder = self.prepare_request(url, method, params).await;

        if let Some(data) = body {
            builder = builder.json(&data);
        }

        let response = self.execute_request(builder).await?;
        let result = response.json::<T>().await?;

        Ok(result)
    }

    /// 获取所有记录
    ///
    /// # 参数
    /// * `T` - 记录类型
    /// * `table_name` - 表名
    /// * `params` - 可选的查询参数
    ///
    /// # 返回
    /// * `ResultE<Vec<T>>` - 记录列表或错误
    pub async fn get_all<R: IRecord>(
        &self,
        table_name: &str,
        params: Option<&ParamsMap>,
    ) -> ResultE<Vec<R>> {
        self.request::<Vec<R>>(table_name, None, Method::GET, None, params)
            .await
    }

    /// 获取单个记录
    ///
    /// # 参数
    /// * `T` - 记录类型
    /// * `table_name` - 表名
    /// * `id` - 记录ID
    ///
    /// # 返回
    /// * `ResultE<T>` - 记录或错误
    pub async fn get<R: IRecord>(
        &self,
        table_name: &str,
        id: &str,
        params: Option<&ParamsMap>,
    ) -> ResultE<R> {
        self.request::<R>(table_name, Some(id), Method::GET, None, params)
            .await
    }

    /// 创建新记录
    ///
    /// # 参数
    /// * `table_name` - 表名
    /// * `record` - 要创建的记录
    ///
    /// # 返回
    /// * `ResultE<T>` - 创建后的记录或错误
    pub async fn post<R: IRecord>(
        &self,
        table_name: &str,
        record: R,
        params: Option<&ParamsMap>,
    ) -> ResultE<R> {
        self.request::<R>(table_name, None, Method::POST, Some(record), params)
            .await
    }

    /// 更新记录
    ///
    /// # 参数
    /// * `table_name` - 表名
    /// * `record` - 要更新的记录，必须包含ID
    ///
    /// # 返回
    /// * `ResultE<R>` - 更新后的记录或错误
    pub async fn put<R: IRecord>(
        &self,
        table_name: &str,
        record: R,
        params: Option<&ParamsMap>,
    ) -> ResultE<R> {
        let id = record.unwrap_id();
        let id_str = id.to_string();

        self.request::<R>(table_name, Some(&id_str), Method::PUT, Some(record), params)
            .await
    }

    /// 删除记录
    ///
    /// # 参数
    /// * `table_name` - 表名
    /// * `id` - 记录ID
    ///
    /// # 返回
    /// * `ResultE<()>` - 成功或错误
    pub async fn delete(
        &self,
        table_name: &str,
        id: &str,
        params: Option<&ParamsMap>,
    ) -> ResultE<()> {
        let url = self.build_url(table_name, Some(id));
        let builder = self.prepare_request(url, Method::DELETE, params).await;
        let _ = self.execute_request(builder).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
