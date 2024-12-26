use chrono::Local;
use std::cmp::min;
use time::{UtcOffset, format_description};
pub use tracing::*;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::fmt::time::OffsetTime;

pub fn init_log(verbose: usize) {
    let n = min(2, verbose);
    let levels = [Level::INFO, Level::DEBUG, Level::TRACE];
    let level = levels[n];

    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]",
    )
    .unwrap();

    let local_time = OffsetTime::new(
        UtcOffset::from_whole_seconds(Local::now().offset().local_minus_utc()).unwrap(),
        format,
    );

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_timer(local_time)
        .finish();

    subscriber::set_global_default(subscriber).expect("TRACING: setting default subscriber failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_option_none() {
        init_log(1);
        info!("hi");
    }
}
