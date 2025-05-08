use rx_db::IVariant;
pub use rx_db::RecordId;
use rx_db::dirdb::DirVariant;
use tokio::sync::Mutex;

use super::common::*;

//#[derive(Default)]
pub struct DaoItem<R> {
    variant: Mutex<DirVariant<R>>,
}

impl<R: Default + Clone + Serialize + DeserializeOwned + ToJSON> DaoItem<R> {
    /// 打开单个数据条目
    pub fn open_name(db_path: &FsPath, var_name: &str, default: Option<R>) -> AnyResult<Self> {
        let default = default.unwrap_or_default();
        let variant = DirVariant::open_path_with_default(db_path, var_name, default).unwrap();
        let variant = Mutex::new(variant);
        Ok(Self { variant })
    }

    /// 获取记录
    pub async fn get(&self) -> Result<CodeResponse<R>> {
        let var = self.variant.lock().await;
        match var.get() {
            Ok(r) => Ok(CodeResponse::Ok(Json(r))),
            Err(_) => Ok(CodeResponse::NotFound),
        }
    }

    pub async fn get_record(&self) -> AnyResult<R> {
        let var = self.variant.lock().await;
        var.get()
    }

    /* 语义上有问题
    /// 删除元素
    pub async fn delete(&self) -> Result<CodeResponse<R>> {
        let mut var = self.variant.lock().await;
        var.delete().unwrap();
        Ok(CodeResponse::NoContent)
    }*/

    /// 更新元素
    pub async fn update(&self, mut record: Json<R>) -> Result<CodeResponse<R>> {
        let mut var = self.variant.lock().await;
        var.set(&mut record.0).unwrap();
        Ok(CodeResponse::Created(record))
    }
}
