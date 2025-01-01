pub use std::path::Path;
pub use path_macro::path;
pub type AnyResult<T> = anyhow::Result<T>;

pub use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
