use std::io::{self, BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::time::Duration;
use std::{fs, thread};

fn return_html_file(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap_or_else(|_| String::new()))
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = match fs::read_to_string("hello.html") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read `hello.html`: {}", e);
            let error_response =
                "HTTP/1.1 404 NOT FOUND\r\n\r\n<html><body><h1>404 Not Found</h1></body></html>";
            stream
                .write_all(error_response.as_bytes())
                .unwrap_or_else(|e| eprintln!("Failed to send error response: {}", e));
            return;
        }
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to send response: {}", e);
    }
}

fn main() {
    let mut port = 7878;
    let mut address = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Enter a port number to start the server (default: 7878) or wait for 5 seconds...");

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();
        if let Ok(port) = trimmed.parse::<u16>() {
            tx.send(Some(port)).unwrap();
        } else {
            tx.send(None).unwrap();
        }
    });

    port = match rx.recv_timeout(Duration::new(5, 0)) {
        Ok(Some(port)) => {
            println!("Using port {}", port);
            7878
        }
        _ => {
            println!("Using default port: 7878");
            7878
        }
    };

    let listener = TcpListener::bind(&address).unwrap_or_else(|_| {
        eprintln!("Failed to bind to {}. Exiting.", address);
        std::process::exit(1);
    });

    println!("Server is running on {}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        return_html_file(stream);
    }
}
