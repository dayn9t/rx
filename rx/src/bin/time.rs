use rx::time::*;

fn main() {
    for i in (0..100).step_by(16) {
        println!("{}", i);
    }

    println!("local_time_str: {}", local_time_str());

    let s = "2000-01-01 00:00:00";

    let ts = Timestamp::new(0);
    println!("Timestamp: {}", ts);

    let ts = Timestamp::parse_from_common_str(s).unwrap();

    println!("Timestamp: {}", ts);

    let dt: NaiveDateTime = ts.into();

    println!("DateTime: {}", dt.to_string());

    let now1: NaiveDateTime = now();

    println!("Now: {}", now1.to_string());

    let t1 = now();
    let t2 = now();
    let d: Duration = t2 - t1;

    println!("D: {}", d);
    //let
}
