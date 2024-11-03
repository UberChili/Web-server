use std::io;
use std::net;

mod server;

fn main() -> io::Result<()> {
    let my_listener = net::TcpListener::bind("127.0.0.1:7878")?;

    for stream in my_listener.incoming() {
        server::handle_client(stream.unwrap());
    }

    Ok(())
}
