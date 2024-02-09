use std::io;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("connected!");
    let mut buf = [0; 512];
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        println!("recv: {}", bytes_read);
        //stream.write(&buf[..bytes_read])?;
        //thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("10.1.0.2:6666").unwrap();
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    println!("Server is running...");
    for stream in listener.incoming() {
        // https://blog.csdn.net/yhc166188/article/details/104016886
        let stream = stream.expect("failed");
        let handle = thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
        });
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        handle.join().unwrap();
    }
    Ok(())
}
