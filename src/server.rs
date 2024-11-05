use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    version: String,
    host: String,
    connection: String,
}

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request = parse_request(http_request).unwrap();
    println!("{:?}", request);

    //let mut response = String::new();
    if request.method == "GET" {
        if request.path.ends_with(".html") {
            let response = html_response(request);
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else if request.path.ends_with(".png")
            || request.path.ends_with(".jpeg")
            || request.path.ends_with(".jpg")
        {
            let response = img_response(request);
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let cont_length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {cont_length}\r\n\r\nContents: {contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }

    //let contents = fs::read_to_string("index.html").unwrap();
    //let cont_lenght = contents.len();

    //let response = format!("HTTP/1.1 200 OK\r\nContent-Lenght: {cont_lenght}\r\n\r\n{contents}");

    //let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
    //stream.write_all(response.as_bytes()).unwrap();
    //stream.flush().unwrap();
}

fn img_response(req: Request) -> String {
    let path = &req.path[1..];
    let content_type = if path.ends_with(".jpeg") || path.ends_with(".jpg") {
        "image/jpg"
    } else if path.ends_with(".png") {
        "image/png"
    } else {
        "application/octect-stream"
    };
    let contents = fs::read(path).unwrap();
    let cont_length = contents.len();

    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        content_type, cont_length
    );

    let mut response = headers.into_bytes();
    response.extend(contents);

    String::from_utf8_lossy(&response).into_owned()
}

fn html_response(req: Request) -> String {
    let path = &req.path[1..];
    let contents = fs::read_to_string(path).unwrap();
    let cont_length = contents.len();

    format!("HTTP/1.1 200 OK\r\nContent-Length: {cont_length}\r\n\r\n{contents}")
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
