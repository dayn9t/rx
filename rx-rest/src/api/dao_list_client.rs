use anyhow::{Result, anyhow};
use reqwest::{Client, header};
use rx_core::prelude::*;
use rx_db::IRecord;
use std::collections::HashMap;
use tokio::sync::Mutex;

/// API客户端返回结果类型
pub type ResultE<T> = Result<T>;

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

    /// 应用认证信息到请求构建器
    async fn apply_auth(&self, builder: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let auth_token = self.auth_token.lock().await;
        match &*auth_token {
            Some(token) => builder.header(header::AUTHORIZATION, format!("Bearer {}", token)),
            None => builder,
        }
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
    pub async fn get_all<T: DeserializeOwned>(
        &self,
        table_name: &str,
        params: Option<&HashMap<String, String>>,
    ) -> ResultE<Vec<T>> {
        let url = self.build_url(table_name, None);

        let client = self.client.lock().await;
        let mut builder = client.get(url);
        builder = self.apply_auth(builder).await;

        if let Some(p) = params {
            builder = builder.query(p);
        }

        let response = builder.send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("API错误: {}", response.status()));
        }

        let records = response.json::<Vec<T>>().await?;
        Ok(records)
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
    pub async fn get<T: DeserializeOwned>(&self, table_name: &str, id: &str) -> ResultE<T> {
        let url = self.build_url(table_name, Some(id));

        let client = self.client.lock().await;
        let builder = client.get(url);
        let builder = self.apply_auth(builder).await;

        let response = builder.send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("API错误: {}", response.status()));
        }

        let record = response.json::<T>().await?;
        Ok(record)
    }

    /// 创建新记录
    ///
    /// # 参数
    /// * `table_name` - 表名
    /// * `record` - 要创建的记录
    ///
    /// # 返回
    /// * `ResultE<T>` - 创建后的记录或错误
    pub async fn post<T: Serialize + DeserializeOwned>(
        &self,
        table_name: &str,
        record: T,
    ) -> ResultE<T> {
        let url = self.build_url(table_name, None);

        let client = self.client.lock().await;
        let builder = client.post(url);
        let builder = self.apply_auth(builder).await;

        let response = builder.json(&record).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("API错误: {}", response.status()));
        }

        let created_record = response.json::<T>().await?;
        Ok(created_record)
    }

    /// 更新记录
    ///
    /// # 参数
    /// * `table_name` - 表名
    /// * `record` - 要更新的记录，必须包含ID
    ///
    /// # 返回
    /// * `ResultE<T>` - 更新后的记录或错误
    pub async fn put<T: IRecord>(&self, table_name: &str, record: T) -> ResultE<T> {
        let id = record.unwrap_id();
        let id_str = id.to_string();
        let url = self.build_url(table_name, Some(&id_str));

        let client = self.client.lock().await;
        let builder = client.put(url);
        let builder = self.apply_auth(builder).await;

        let response = builder.json(&record).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("API错误: {}", response.status()));
        }

        let updated_record = response.json::<T>().await?;
        Ok(updated_record)
    }

    /// 删除记录
    ///
    /// # 参数
    /// * `table_name` - 表名
    /// * `id` - 记录ID
    ///
    /// # 返回
    /// * `ResultE<()>` - 成功或错误
    pub async fn delete(&self, table_name: &str, id: &str) -> ResultE<()> {
        let url = self.build_url(table_name, Some(id));

        let client = self.client.lock().await;
        let builder = client.delete(url);
        let builder = self.apply_auth(builder).await;

        let response = builder.send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("API错误: {}", response.status()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
