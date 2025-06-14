use crate::dirdb::DirDb;
//use crate::redisdb::RedisDb;
//use crate::{IDatabase, dirdb, redisdb};
use crate::{IDatabase, dirdb};
use anyhow::anyhow;
use rx_core::prelude::*;
use url::Url;

pub fn remove_table(url: &str, name: &str) -> AnyResult<()> {
    let uri = Url::parse(url)?;
    let r = match uri.scheme() {
        dirdb::SCHEME => {
            let db = DirDb::open(url)?;
            db.remove_table(name)?
        }
        /*redisdb::SCHEME => {
            let db = RedisDb::open(url)?;
            db.remove_table(name)?
        }*/
        _ => return Err(anyhow!("Invalid scheme")),
    };
    Ok(r)
}

pub fn remove_variant(url: &str, name: &str) -> AnyResult<()> {
    let uri = Url::parse(url)?;
    let r = match uri.scheme() {
        dirdb::SCHEME => {
            let db = DirDb::open(url)?;
            db.remove_variant(name)?
        }
        /*redisdb::SCHEME => {
            let db = RedisDb::open(url)?;
            db.remove_variant(name)?
        }*/
        _ => return Err(anyhow!("Invalid scheme")),
    };
    Ok(r)
}
