use std::env;

fn main() {
    for (key, value) in env::vars() {
        println!("\t{}: {}", key, value);
    }
}
