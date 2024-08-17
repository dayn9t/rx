#![feature(duration_constructors)]

use path_macro::path;
use rx_core::log::{info, init_log};
use rx_core::time::{NaiveDateTime, Utc};
use rx_net::frp::*;
use std::time::Duration;

fn main() {
    init_log(0);

    //let start_time: NaiveDateTime = (Utc::now() - Duration::from_days(3)).naive_utc();
    let start_time = NaiveDateTime::default();
    info!("Start time: {}", start_time);

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

    //let nodes = stat.get_new_nodes(start_time);
    let nodes = stat.get_all_nodes();
    info!("nodes len: {}", nodes.len());
    let mut names: Vec<_> = nodes.iter().map(|n| n.name[n.name.len() - 3..].to_string()).collect();
    names.sort();
    for name in names {
        println!("{}", name);
    }
}
