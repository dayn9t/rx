use std::io::{self,Write};
use std::net::TcpStream;

 
 
fn main() -> io::Result<( )> {
    let mut stream = TcpStream::connect("10.1.0.2:6666")?;
    for i in 0..3 {
        println!("i = {}", i);
        let mut input = String::new();
        println!("read_line:");
        io::stdin().read_line(&mut input).expect("Failed to read");
        //println!("write");
        stream.write(input.as_bytes()).expect("failed to write");
 
        println!("OK");
    }
    Ok(())
}
