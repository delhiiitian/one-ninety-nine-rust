// Starts the real server on a free port and talks to it over TCP,
// including many clients at once.
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::thread;

struct Server {
    child: Child,
    addr: String,
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn start() -> Server {
    // Ask the OS for a free port, then hand it to the server.
    let port = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    let mut child = Command::new(env!("CARGO_BIN_EXE_kv-server"))
        .args(["--port", &port.to_string(), "--threads", "4"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("server should start");
    // Wait for the "Listening on" line so we know it is ready.
    let mut out = BufReader::new(child.stdout.take().unwrap());
    let mut line = String::new();
    out.read_line(&mut line).unwrap();
    assert!(line.starts_with("Listening on"), "got: {line}");
    // Keep reading the request log in the background so the pipe stays open.
    thread::spawn(move || for _ in out.lines() {});
    Server { child, addr: format!("127.0.0.1:{port}") }
}

fn send(addr: &str, method: &str, path: &str, body: &str) -> (u16, String) {
    let mut s = TcpStream::connect(addr).unwrap();
    write!(s, "{method} {path} HTTP/1.1\r\nHost: test\r\nContent-Length: {}\r\n\r\n{body}", body.len()).unwrap();
    let mut resp = String::new();
    s.read_to_string(&mut resp).unwrap();
    let status = resp.split_whitespace().nth(1).unwrap().parse().unwrap();
    let body = resp.split_once("\r\n\r\n").unwrap().1.to_string();
    (status, body)
}

#[test]
fn crud_over_the_network() {
    let srv = start();
    let a = srv.addr.as_str();
    assert_eq!(send(a, "GET", "/health", ""), (200, r#"{"status":"ok"}"#.into()));
    assert_eq!(send(a, "PUT", "/keys/city", "Pune").0, 201);
    assert_eq!(send(a, "GET", "/keys/city", ""), (200, r#"{"key":"city","value":"Pune"}"#.into()));
    assert_eq!(send(a, "DELETE", "/keys/city", "").0, 200);
    assert_eq!(send(a, "GET", "/keys/city", "").0, 404);
    assert_eq!(send(a, "GET", "/nowhere", "").0, 404);
}

#[test]
fn many_clients_at_once() {
    let srv = start();
    let addr = srv.addr.clone();
    let clients: Vec<_> = (0..20)
        .map(|i| {
            let addr = addr.clone();
            thread::spawn(move || send(&addr, "PUT", &format!("/keys/item-{i}"), &format!("value {i}")).0)
        })
        .collect();
    for c in clients {
        assert_eq!(c.join().unwrap(), 201);
    }
    let (status, body) = send(&addr, "GET", "/stats", "");
    assert_eq!(status, 200);
    assert!(body.starts_with(r#"{"keys":20,"#), "got {body}");
    assert_eq!(send(&addr, "GET", "/keys/item-7", "").1, r#"{"key":"item-7","value":"value 7"}"#);
}

#[test]
fn rejects_a_body_that_is_too_large() {
    let srv = start();
    let mut s = TcpStream::connect(&srv.addr).unwrap();
    write!(s, "PUT /keys/big HTTP/1.1\r\nContent-Length: 999999\r\n\r\n").unwrap();
    let mut resp = String::new();
    s.read_to_string(&mut resp).unwrap();
    assert!(resp.starts_with("HTTP/1.1 413"), "got {resp}");
}
