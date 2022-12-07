use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;


fn main() {
    let ip = "0.0.0.0";
    let port = "6969";

    let socket =
        TcpListener::bind(format!("{}:{}", ip, port)).expect("unable to read from socket");
    
    println!("Webserver is running at {}:{}", ip, port);
    println!("");
    
    for data in socket.incoming() {
        let data = data.expect("unable to read from socket");
    
        handleconnection(data);
    }
}

fn handleconnection(mut data: TcpStream) {
    let mut buffer: [u8;2024] = [0;2024];
    
    data.read(&mut buffer).expect("unable to read from socket");
    let request_data = String::from_utf8_lossy(&buffer);
    let request = HttpRequest::new(request_data.to_string());

    // testing
    println!("request.url: {}", request.uri);

    let base_dir = "./html";

    let response = if request.method == "GET" {

        let filename: String = if request.uri == "/" {
            "index.html".to_string()
        }
        else if request.uri.contains("?v=") {
            request.uri.split("?v=").take(1).collect()
        }
        else {
            request.uri.to_string()
        };
        
        let path: String = format!("{}/{}", base_dir, filename);

        if Path::new(&path).exists() {
            let content = fs::read_to_string(&path).expect("unable to read");

            let mime_type =
                Path::new(&path).extension().expect("unable to read extension").to_string_lossy();

                let mime_type = if mime_type == "js" {
                    "javascript".to_string()
                } else {
                    mime_type.to_string()
                };
            
            let content_type = format!("text/{}", mime_type);

            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {content_length}\r\nContent-Type: {content_type}\r\n\r\n{body}",
                content_length=content.len(),
                content_type=content_type,
                body=content
            )

        } else {
            "HTTP/1.1 404 NOT FOUND\r\n\r\nNot Found".to_string()
        }

    } else {
        "HTTP/1.1 500 Internal Server Error\r\n\r\nIn Progress".to_string()
    };
    
    data.write(response.as_bytes()).expect("unable to write response data");
    data.flush().expect("unable to flush response data");

    println!(
        "Request: {}",
        String::from_utf8_lossy(&buffer[..])
    );
}

#[derive(Debug)]
struct HttpRequest {
    method: String,
    uri: String,
    version: String,
    headers: HashMap<String, String>,
    body: String
}

impl HttpRequest {
    fn new(request_data: String) -> Self {
        let r: Vec<&str> = request_data.splitn(2, "\r\n\r\n").collect();
        let request_data = r[0];
        let body = r[1].to_string();

        let r: Vec<&str> = request_data.splitn(2, "\r\n").collect();
        let status_line = r[0];

        let s: Vec<&str> = status_line.split(" ").collect();
        let method = s[0].to_string();
        let uri = s[1].to_string();
        let version = s[2].to_string();

        let header_raw_data = r[1];
        let header_data: Vec<&str> = header_raw_data.split("\r\n").collect();
        let mut headers: HashMap<String, String> = HashMap::new();

        for header in header_data {
            let key_value: Vec<&str> = header.splitn(2,":").collect();
            headers.insert(key_value[0].to_string(), key_value[1].to_string());
        }

        HttpRequest { method, uri, version, headers, body }
    }
}