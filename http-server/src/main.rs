use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Listening on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let mut temp_vec = handle_connection(stream);
        //iterate over the vec and call the parse request function
        for line in &temp_vec {
            parse_request(line);
        }
        send_response(&mut stream, "200 OK", "Hello World!");
    }
}

fn handle_connection(stream: TcpStream) -> Vec<String> {
    let reader = BufReader::new(&stream);
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

fn parse_request(line: &str) -> Option<()> {
    let mut parts = line.split_whitespace();
    let method = parts.next()?;
    let path = parts.next()?;
    println!("method is {}, and parts is {}", method, path);
    Some(())
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
