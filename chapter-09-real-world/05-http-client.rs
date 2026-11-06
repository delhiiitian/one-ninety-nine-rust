// Chapter 9, Lesson 5: Speaking HTTP by hand
// Run: cargo run --example ch09-05

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// HTTP is just text over TCP, like a letter with a fixed format:
// a request line, headers, a blank line, then the body.
fn get(addr: &str, path: &str) -> std::io::Result<(u16, String)> {
    let mut stream = TcpStream::connect(addr)?;
    write!(stream, "GET {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    let status: u16 = response.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0);
    let body = response.split("\r\n\r\n").nth(1).unwrap_or("").to_string();
    Ok((status, body))
}

// A minimal web server so the client has something to talk to offline.
fn serve(listener: TcpListener, requests: usize) {
    for stream in listener.incoming().take(requests) {
        let mut stream = stream.unwrap();
        let mut reader = BufReader::new(&stream);
        let mut first = String::new();
        reader.read_line(&mut first).unwrap();
        let path = first.split_whitespace().nth(1).unwrap_or("/").to_string();
        // Read and ignore the rest of the headers, up to the blank line.
        let mut header = String::new();
        while reader.read_line(&mut header).unwrap() > 2 {
            header.clear();
        }
        let (status, body) = match path.as_str() {
            "/" => ("200 OK", "Welcome to the Rust 199 shop".to_string()),
            "/price/chai" => ("200 OK", "chai costs Rs 20".to_string()),
            _ => ("404 Not Found", format!("no page at {path}")),
        };
        write!(stream, "HTTP/1.1 {status}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{body}", body.len()).unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?.to_string();
    let server = thread::spawn(move || serve(listener, 3));

    for path in ["/", "/price/chai", "/price/gold"] {
        let (status, body) = get(&addr, path)?;
        println!("GET {path:<12} -> {status} {body}");
    }
    server.join().unwrap();
    // Real projects use a crate such as `reqwest` or `ureq`, which adds
    // HTTPS, redirects and JSON. The idea underneath is the same.
    Ok(())
}
