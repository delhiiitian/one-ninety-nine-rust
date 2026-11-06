//! Just enough HTTP/1.1 to talk to curl and browsers.

use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Read, Write};

/// Largest body we accept, so one client cannot fill our memory.
pub const MAX_BODY: usize = 64 * 1024;

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn json(status: u16, value: serde_json::Value) -> Response {
        Response { status, body: value.to_string() }
    }

    fn reason(&self) -> &'static str {
        match self.status {
            200 => "OK",
            201 => "Created",
            400 => "Bad Request",
            404 => "Not Found",
            405 => "Method Not Allowed",
            413 => "Payload Too Large",
            _ => "Internal Server Error",
        }
    }

    /// Writes the status line, headers, a blank line, then the body.
    pub fn write_to(&self, out: &mut impl Write) -> io::Result<()> {
        write!(
            out,
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            self.status,
            self.reason(),
            self.body.len(),
            self.body
        )?;
        out.flush()
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Empty,
    BadRequestLine(String),
    BodyTooLarge(usize),
    Io(String),
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::Io(e.to_string())
    }
}

/// Reads one request from any reader (a TcpStream, or bytes in a test).
pub fn read_request(input: impl Read) -> Result<Request, ParseError> {
    let mut reader = BufReader::new(input);

    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        return Err(ParseError::Empty);
    }
    let parts: Vec<&str> = line.split_whitespace().collect();
    let [method, path, version] = parts[..] else {
        return Err(ParseError::BadRequestLine(line.trim().to_string()));
    };
    if !version.starts_with("HTTP/") {
        return Err(ParseError::BadRequestLine(line.trim().to_string()));
    }
    let (method, path) = (method.to_string(), path.to_string());

    // Headers: "Name: value" lines until a blank line.
    let mut headers = HashMap::new();
    loop {
        let mut h = String::new();
        if reader.read_line(&mut h)? == 0 || h.trim().is_empty() {
            break;
        }
        if let Some((name, value)) = h.split_once(':') {
            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }

    // Body: exactly Content-Length bytes.
    let len: usize = headers.get("content-length").and_then(|v| v.parse().ok()).unwrap_or(0);
    if len > MAX_BODY {
        return Err(ParseError::BodyTooLarge(len));
    }
    let mut body = vec![0; len];
    reader.read_exact(&mut body)?;
    let body = String::from_utf8_lossy(&body).into_owned();

    Ok(Request { method, path, headers, body })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_put_with_body() {
        let raw = "PUT /keys/city HTTP/1.1\r\nHost: x\r\nContent-Length: 4\r\n\r\nPune";
        let req = read_request(raw.as_bytes()).unwrap();
        assert_eq!(req.method, "PUT");
        assert_eq!(req.path, "/keys/city");
        assert_eq!(req.headers["host"], "x");
        assert_eq!(req.body, "Pune");
    }

    #[test]
    fn rejects_garbage_and_huge_bodies() {
        assert_eq!(read_request("".as_bytes()), Err(ParseError::Empty));
        assert!(matches!(read_request("hello\r\n".as_bytes()), Err(ParseError::BadRequestLine(_))));
        let huge = format!("PUT /k HTTP/1.1\r\nContent-Length: {}\r\n\r\n", MAX_BODY + 1);
        assert!(matches!(read_request(huge.as_bytes()), Err(ParseError::BodyTooLarge(_))));
    }

    #[test]
    fn writes_a_response() {
        let mut out = Vec::new();
        Response::json(404, serde_json::json!({"error": "nope"})).write_to(&mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        assert!(text.starts_with("HTTP/1.1 404 Not Found\r\n"));
        assert!(text.ends_with("\r\n\r\n{\"error\":\"nope\"}"));
    }
}
