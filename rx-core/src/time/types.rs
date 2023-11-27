pub use chrono::prelude::*;
pub use chrono::ParseResult;

pub type UtcDateTime = DateTime<chrono::prelude::Utc>;
pub type LocalDateTime = DateTime<chrono::prelude::Local>;

pub use time::Duration;
