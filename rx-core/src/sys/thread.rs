/// 随机等待(秒)
pub fn rand_wait_secs(max_delay: u64) {
    use rand::Rng;
    use std::time::Duration;
    let mut rng = rand::thread_rng();
    let wait = rng.gen_range(1..max_delay);
    std::thread::sleep(Duration::from_secs(wait));
}
