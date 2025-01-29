use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;
use std::fs;

fn get_request_get_path(request: &str) -> Option<String> {      // Shoutout chatgpt🙏
    request.lines()
    .next() // Get the first line of the request
    .and_then(|line| line.split_whitespace().nth(1)) // Get the URL path
    .map(String::from) // Convert &str to String
}

fn send_file(stream: &mut TcpStream, file_path: String) {
    match fs::exists(); // check if file exists and send it through stream if it does.
    let contents = fs::read_to
}

fn handle_client(stream: &mut TcpStream) {
    let request_ip = stream.peer_addr().unwrap().to_string();
    println!("Client connected, IP: {request_ip}");

    let mut buffer = [0; 1024];
    let mut file_path: String = String::new();
    match stream.read(&mut buffer) {
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
            match get_request_get_path(request_str) {
                Some(_file_path) => file_path = _file_path,
                None => eprintln!("Could not extract filepath from GET request"),
            };
        },
    }

    println!("Request for {file_path} received!");

    let _ = stream.write(b"HTTP/1.1 200 OK\r\n\r\n");
    if file_path == String::from("/") {
        send_file(stream, String::from("/index.html")); // and index.html
    } else {
        let _ = stream.write(b"<h1>empty</h1>");
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("192.168.2.120:8080")?;
    println!("Listening on 192.168.2.120:8080");

    // Do stuff when someone requests stuff
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_client(&mut stream);
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {e}");
            }
        }
    }
    Ok(())
}
