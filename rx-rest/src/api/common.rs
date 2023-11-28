pub use std::path::Path;

pub use poem::Result;
pub use poem_openapi::{
    param::Path as UrlPath, payload::Json, types::ToJSON, ApiResponse, Object
};

pub use rx_core::log::*;
pub use rx_core::text::{BoxResult, Deserialize, DeserializeOwned, Serialize};

#[derive(ApiResponse)]
pub enum CodeResponse<R: Send + ToJSON> {
    /// [GET]：服务器成功返回用户请求的数据，幂等操作
    #[oai(status = 200)]
    Ok(Json<R>),
    /// [POST/PUT/PATCH]：用户新建或修改数据成功
    #[oai(status = 201)]
    Created(Json<R>),
    /// [DELETE]：用户删除数据成功
    #[oai(status = 204)]
    NoContent,
    /// [POST/PUT/PATCH]：用户发出的请求有错误，该幂等操作
    #[oai(status = 400)]
    InvalidRequest,
    /// [*]：表示用户没有权限（令牌、用户名、密码错误）
    #[oai(status = 401)]
    Unauthorized,
    /// [*] 表示用户得到授权，但是访问是被禁止的
    #[oai(status = 403)]
    Forbidden,
    /// [*]：用户发出的请求针对的是不存在的记录，幂等操作
    #[oai(status = 404)]
    NotFound,
    /// [*]：服务器发生错误，无法判断发出的请求是否成功
    #[oai(status = 500)]
    InternalServerError,
}
