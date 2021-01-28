use rx::time::*;


fn main() {
    println!("local_time_str: {}", local_time_str());

    let s = "2000-01-01 00:00:00";

    let ts = Timestamp::new(0);
    println!("Timestamp: {}", ts);

    let ts = Timestamp::parse_from_common_str(s).unwrap();

    println!("Timestamp: {}", ts);

    let dt: DateTime = ts.into();

    println!("DateTime: {}", dt.to_string());

    let now: DateTime = now();

    println!("Now: {}", now.to_string());
}
