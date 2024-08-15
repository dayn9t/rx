#![feature(duration_constructors)]

use path_macro::path;
use rx_core::log::info;
use rx_core::time::{NaiveDateTime, Utc};
use rx_net::frp::*;
use std::time::Duration;

fn main() {
    let files = [
        "stdout.log",
        "stdout.log.1",
        "stdout.log.2",
        "stdout.log.3",
        "stdout.log.4",
        "stdout.log.5",
    ];
    let dir = path!("/var/log/frps");

    let mut stat = TimeStat::new();

    for file in files.iter() {
        let file = path!(dir / file);
        info!("Update with log: {}", file.display());
        stat.update_with_log(&file).unwrap()
    }

    let start_time: NaiveDateTime = (Utc::now() - Duration::from_days(3)).naive_utc();
    stat.get_new_nodes(start_time)
        .iter()
        .for_each(|node| info!("New node: {:?}", node));
}
