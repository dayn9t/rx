use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let m: Option<String> = con.get("student_meta")?;
    println!("{:?}", m);

    println!("test del");
    let _: () = con.del("student_meta")?;
    println!("{:?}", m);

    println!("test hexists");
    let exists: bool = con.hexists("my_key11", 2)?;
    println!("{:?}", exists);

    println!("test hlen");
    let len: usize = con.hlen("my_key11")?;
    println!("{:?}", len);

    println!("test hkeys");
    let keys: Vec<usize> = con.hkeys("my_key11")?;
    println!("{:?}", keys);

    let _: () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

fn main() {
    fetch_an_integer().unwrap();
}
