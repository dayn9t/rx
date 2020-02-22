
extern crate cx;
use cx::fs::*;


fn main() {
    let p = Path::new("/home/jiang/note");

    let files = files_in(&p, &"md");
    println!("{:?}", files);
}
