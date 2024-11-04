use std::{
    fmt::Display,
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
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

pub fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    //println!("Request: {:#?}", http_request);

    let request = parse_request(http_request).unwrap();
    println!("{}", request);

    println!("So how do we send or write a response?");

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_request(req: Vec<String>) -> Result<Request, io::Error> {
    if req.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Empty request"));
    }

    let mut request = Request {
        method: String::new(),
        path: String::new(),
        version: String::new(),
        host: String::new(),
        connection: String::new(),
    };

    for line in req {
        if line.contains("HTTP") {
            let mut parts = line.split_whitespace();
            request.method = parts.next().unwrap_or("").to_string();
            request.path = parts.next().unwrap_or("").to_string();
            request.version = parts.next().unwrap_or("").to_string();
        } else if line.starts_with("Host:") {
            request.host = line.split_whitespace().nth(1).unwrap_or("").to_string();
        } else if line.starts_with("Connection:") {
            request.connection = line.split_whitespace().nth(1).unwrap_or("").to_string();
        }
    }
    Ok(request)
}
