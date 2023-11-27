use std::cmp::min;

pub use tracing::*;
use tracing_subscriber::FmtSubscriber;

pub fn init_log(verbose: usize) {
    let n = min(2, verbose);
    let levels = [Level::INFO, Level::DEBUG, Level::TRACE];
    let level = levels[n];
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();

    subscriber::set_global_default(subscriber).expect("TRACING: setting default subscriber failed");
}
