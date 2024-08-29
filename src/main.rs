// https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// TODO: I am still not entirely happy with how it works. There is got to be a much more simple way. 
struct FilePaths<'a> {
    index: &'a str,
    not_found: &'a str, 
}

fn main() {
    // bind() here works like TcpListener listener = new TcpListener().
    // Here is unwrap used to ensure an error is returned whenever something goes wrong.
    // Yet one should use try-catch instead.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // lines() is useful here because it splits data into lines whenever it sees newline byte.
    let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();

    let request_line = match http_request.get(0) {
        Some(line) => line,
        None => return,
    };

    let paths = FilePaths {index: "./templates/index.html", not_found: "./templates/404.html"}; 

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", paths.index),
        "GET /sleep HTTP/1.1" => {
           thread::sleep(Duration::from_secs(5));
           ("HTTP/1.1 200 OK", paths.index)
        }
        _ => ("HTTP/1.1 404 NOT FOUND", paths.not_found),
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| "File not found".to_string());
    let length = contents.len();

    // If removed server will return plain text.
    let content_type = match filename {
        paths.index | paths.not_found => "text/html",
        _ => "text/plain",
    };

    let response = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}

