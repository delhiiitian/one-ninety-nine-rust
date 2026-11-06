//! kv-server: run it, then talk to it with curl.
//!
//!   cargo run -p kv-server -- --port 7878 --threads 4
//!   curl -X PUT -d Pune localhost:7878/keys/city
//!   curl localhost:7878/keys/city

use kv_server::http::{read_request, ParseError, Response, MAX_BODY};
use kv_server::pool::ThreadPool;
use kv_server::router::route;
use kv_server::store::Store;
use serde_json::json;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

struct Config {
    port: u16,
    threads: usize,
}

fn parse_args(args: &[String]) -> Result<Config, String> {
    let mut cfg = Config { port: 7878, threads: 4 };
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        let value = it.next().ok_or(format!("{flag} needs a value"))?;
        match flag.as_str() {
            "--port" => cfg.port = value.parse().map_err(|_| format!("bad port: {value}"))?,
            "--threads" => {
                cfg.threads = value.parse().map_err(|_| format!("bad thread count: {value}"))?;
                if cfg.threads == 0 {
                    return Err("--threads must be at least 1".into());
                }
            }
            other => return Err(format!("unknown option {other}")),
        }
    }
    Ok(cfg)
}

fn handle(mut stream: TcpStream, store: &Store) {
    // A slow or silent client must not hold a worker forever.
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let response = match read_request(&stream) {
        Ok(req) => {
            let res = route(&req, store);
            // A log line must never crash a worker, so write errors are ignored.
            let _ = writeln!(std::io::stdout(), "{} {} -> {}", req.method, req.path, res.status);
            res
        }
        Err(ParseError::BodyTooLarge(n)) => {
            Response::json(413, json!({"error": format!("body of {n} bytes is over {MAX_BODY}")}))
        }
        Err(ParseError::Empty) => return, // client connected and left
        Err(e) => Response::json(400, json!({"error": format!("{e:?}")})),
    };
    let _ = response.write_to(&mut stream);
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cfg = match parse_args(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}\nusage: kv-server [--port N] [--threads N]");
            std::process::exit(2);
        }
    };

    let listener = match TcpListener::bind(("127.0.0.1", cfg.port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("error: cannot listen on port {}: {e}", cfg.port);
            std::process::exit(1);
        }
    };
    let pool = ThreadPool::new(cfg.threads);
    let store = Store::new();
    println!("Listening on http://{} with {} workers", listener.local_addr().unwrap(), pool.size());
    let _ = std::io::stdout().flush();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let store = store.clone(); // cheap: clones two Arcs
                pool.execute(move || handle(stream, &store));
            }
            Err(e) => eprintln!("connection failed: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    #[test]
    fn parses_options() {
        let c = parse_args(&args("--port 9000 --threads 2")).unwrap();
        assert_eq!((c.port, c.threads), (9000, 2));
        assert!(parse_args(&args("--threads 0")).is_err());
        assert!(parse_args(&args("--port")).is_err());
        assert!(parse_args(&args("--colour red")).is_err());
    }
}
