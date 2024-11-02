use std::{
    io::{self, Read},
    net::{self, TcpStream},
};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("Received connection: {:?}", stream);

    let mut request: Vec<u8> = Vec::new();
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                request.extend_from_slice(&buffer[..n]);
            }
            Err(e) => return Err(e),
        }
    }

    match String::from_utf8(request) {
        Ok(request_str) => println!("Raw request:\n{}", request_str),
        Err(e) => println!("Invalid UTF-8 sequence: {}", e),
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let my_listener = net::TcpListener::bind("127.0.0.1:8080")?;

    for stream in my_listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
