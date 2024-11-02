use std::{
    fmt::Display,
    io::{self, BufRead, BufReader},
    net::{self, TcpStream},
};

struct Request {
    method: String,
    path: String,
    version: String,
    host: String,
    connection: String,
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Request {{",)?;
        writeln!(f, " Method: {}", self.method)?;
        writeln!(f, " Path: {}", self.path)?;
        writeln!(f, " Version: {}", self.version)?;
        writeln!(f, " Host: {}", self.host)?;
        writeln!(f, " Connection: {}", self.connection)?;
        write!(f, "}}")
    }
}

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    //println!("Request: {:#?}", http_request);

    let request = parse_request(http_request).unwrap();
    println!("{}", request)
}

fn parse_request(req: Vec<String>) -> Result<Request, io::Error> {
    let mut method: String = String::new();
    let mut path: String = String::new();
    let mut version: String = String::new();
    let mut host: String = String::new();
    let mut connection: String = String::new();

    for line in req {
        if line.contains("HTTP") {
            let method_line: Vec<_> = line
                .split_whitespace()
                .map(|word| word.to_string())
                .collect();
            method = method_line[0].clone();
            path = method_line[1].clone();
            version = method_line[method_line.len() - 1].clone();
        } else if line.contains("Host:") {
            let host_line: Vec<_> = line
                .split_whitespace()
                .map(|word| word.to_string())
                .collect();
            host = host_line[host_line.len() - 1].clone();
        } else if line.contains("Connection:") {
            let connection_line: Vec<_> = line
                .split_whitespace()
                .map(|word| word.to_string())
                .collect();
            connection = connection_line[connection_line.len() - 1].clone();
        }
    }
    Ok(Request {
        method,
        path,
        version,
        host,
        connection,
    })
}

fn main() -> io::Result<()> {
    let my_listener = net::TcpListener::bind("127.0.0.1:7878")?;

    for stream in my_listener.incoming() {
        handle_client(stream.unwrap());
    }

    Ok(())
}
