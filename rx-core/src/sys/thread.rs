use std::time::Duration;

use rand::Rng;

/// 随机等待(秒)
pub fn rand_wait_secs(max_delay: u64) {
    let mut rng = rand::rng();
    let wait = rng.random_range(1..max_delay + 1);
    std::thread::sleep(Duration::from_secs(wait));
}

/// 等待(秒)
pub fn wait_secs(secs: u64) {
    std::thread::sleep(Duration::from_secs(secs));
}
