use std::io::prelude::*;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for incoming in listener.incoming() {

        if let Ok(mut stream) = incoming {
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();

            println!("Steam read:");
            println!("{}", String::from_utf8_lossy(&buffer));

            println!("Responding: Hello");
            let _ = stream.write("HTTP/1.1 200 OK\r\n\r\nHello\r\n".as_bytes());

            println!("Response sent!");
        }
    }
}
