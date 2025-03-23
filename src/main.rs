use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::path::Path;

fn get_request_get_path(request: &str) -> Option<String> {      // Shoutout chatgpt🙏
    request.lines()
        .next() // Get the first line of the request
        .and_then(|line| line.split_whitespace().nth(1)) // Get the URL path
        .map(String::from) // Convert &str to String
}

fn send_file(stream: &mut TcpStream, file_path_param: String) {
    let mut file_path = file_path_param.clone();
    if file_path.ends_with("/") {
        send_file(stream, file_path + "index.html");
        return;
    }

    file_path.remove(0); // If filename doesn't end in /, remove the first character (which is /)

    if !Path::new(&file_path).exists() {
        let _ = stream.write_all(b"HTTP/1.1 404 NOT FOUND\r\nContent-Length: 22\r\n\r\n<h1>404 Not Found</h1>");
        return;
    }
    if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") || file_path.ends_with(".png") || file_path.ends_with(".gif") || file_path.ends_with(".svg") {
        let file_content = match fs::read(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read file: {e}");
                return;
            },
        };
        let response_header = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: image/{}\r\n\r\n",
            file_content.len(),
            if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
                "jpeg"
            } else if file_path.ends_with(".png") {
                "png"
            } else if file_path.ends_with(".svg") {
                "svg+xml"
            } else {
                "gif"
            }
        );
        let _ = stream.write_all(response_header.as_bytes());
        let _ = stream.write_all(&file_content);
        return;
    }

    let file_content = match fs::read(&file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file: {e}");
            return;
        },
    };

    let response_header = if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") || file_path.ends_with(".png") || file_path.ends_with(".gif") || file_path.ends_with(".svg") {
        let file_content = match fs::read(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to read file: {e}");
                return;
            },
        };
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: image/{}\r\n\r\n",
            file_content.len(),
            if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
                "jpeg"
            } else if file_path.ends_with(".png") {
                "png"
            } else if file_path.ends_with(".svg") {
                "svg+xml"
            } else {
                "gif"
            }
        )
    } else {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
            file_content.len()
        )
    };

    let _ = stream.write_all(response_header.as_bytes());
    let _ = stream.write_all(&file_content);
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

    send_file(stream, file_path);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Listening on 127.0.0.1:8080");

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
