use tokio::sync::Mutex;

pub use rx_db::RecordId;
use rx_db::{DirDb, DirVariant, IVariant};

use super::common::*;

//#[derive(Default)]
pub struct DaoItem<R> {
    variant: Mutex<DirVariant<R>>,
}

impl<R: Default + Clone + Serialize + DeserializeOwned + ToJSON> DaoItem<R> {
    /// 打开单个数据条目
    pub fn open_name<P, S>(db_path: P, var_name: &S, default: Option<R>) -> BoxResult<Self>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let db = DirDb::open(db_path).unwrap();
        let variant = Mutex::new(DirVariant::open(&db, var_name, default).unwrap());
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
