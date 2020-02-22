
#[macro_use]
extern crate cx;


fn fun() -> bool {

    let r: Result<i32, String> = Ok(1);
    let v = try_or_false!(r);
    false
}

fn main() {
    fun();
}
