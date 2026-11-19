use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Listening on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut temp_vec = handle_connection(&mut stream);
        //iterate on only the first line
        if let Some(request_line) = temp_vec.first() {
            if let Some((method, path)) = parse_request(request_line) {
                match (method, path) {
                    ("GET", "/") => send_response(&mut stream, "200 OK", "<h1>Home</h1>"),
                    ("GET", "/hello") => send_response(&mut stream, "200 OK", "<h1>Hello!</h1>"),
                    _ => send_response(&mut stream, "404 NOT FOUND", "<h1>Page not found</h1>"),
                }
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) -> Vec<String> {
    let reader = BufReader::new(&mut *stream);
    let mut my_vec: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            break;
        }
        my_vec.push(line);
    }
    my_vec
}

fn parse_request(line: &str) -> Option<(&str, &str)> {
    let mut parts = line.split_whitespace();
    let method = parts.next()?;
    let path = parts.next()?;
    Some((method, path))
}

fn send_response(stream: &mut TcpStream, status: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status,
        body.len(),
        body
    );
    stream.write_all(response.as_bytes()).unwrap();
}
