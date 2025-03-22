use std::env;

fn main() {
    for (key, value) in env::vars() {
        println!("\t{}: {}", key, value);
    }
    let s = "大黄";
    println!("{} size: {}", s, s.chars().count());
}
