// Chapter 9, Lesson 4: A tiny TCP server
// Run: cargo run --example ch09-04

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// Handles one connection: reads a line and replies in upper case.
fn handle(stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = stream;
    let mut line = String::new();
    reader.read_line(&mut line)?;
    writeln!(writer, "ECHO: {}", line.trim().to_uppercase())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    // A server is like a shop counter: it waits at an address (the port)
    // and serves each customer who walks up. Port 0 asks the OS for any
    // free port, so this lesson never clashes with another program.
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    println!("Server listening on {addr}");

    // Serve exactly two customers in a background thread, then stop.
    let server = thread::spawn(move || {
        for stream in listener.incoming().take(2) {
            if let Err(e) = handle(stream.expect("bad connection")) {
                eprintln!("error: {e}");
            }
        }
    });

    // Act as two customers so the lesson runs on its own.
    for msg in ["namaste", "hello rust"] {
        let mut client = TcpStream::connect(addr)?;
        writeln!(client, "{msg}")?;
        let mut reply = String::new();
        BufReader::new(client).read_line(&mut reply)?;
        println!("Sent {msg:?}, got {:?}", reply.trim());
    }
    server.join().unwrap();
    println!("Server closed");
    Ok(())
}
