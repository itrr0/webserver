use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let request_ip = stream.peer_addr().unwrap().to_string();
    println!("Client connected, IP: {request_ip}");

    let mut buffer = [0; 1024];
    let mut filename: String = String::new();
    match stream.read(&mut buffer) {        // FIX THIS BULLSHIT
        Err(e) => {
            eprintln!("Failed to read request: {e}");
        },
        Ok(_) => {
            let request_str = match std::str::from_utf8(&buffer) {
                Ok(str) => str,
                Err(_) => {
                    eprintln!("Failed to parse request as UTF-8");
                    return;
                }
            };
            if !request_str.starts_with("GET ") {
                eprintln!("Invalid request: does not start with 'GET'");
                return;
            }
            filename = request_str.rsplit('/').next().map(|s| s.to_string()).unwrap_or_default();
        },
    }

    println!("Request received:\n{filename}");

    let _ = stream.write(b"HTTP/1.1 200 OK\r\nContent-Length: 48\r\n\r\n<h1>Hello, world!</h1>");
    let _ = stream.write(b"<h2>New bit of text...<h2>");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("192.168.2.120:8080")?;
    println!("Listening on 192.168.2.120:8080");

    // Do stuff when someone requests stuff
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {e}");
            }
        }
    }
    Ok(())
}
