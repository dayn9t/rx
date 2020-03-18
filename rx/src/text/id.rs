pub use uuid::Uuid;

/// 从字符串中解析/获取UUID
pub fn uuid_or_new(name: &str) -> Uuid {
    if let Ok(uuid) = Uuid::parse_str(name) {
        uuid
    } else {
        Uuid::new_v5(&uuid::NAMESPACE_OID, name)
    }
}
