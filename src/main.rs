use std::{
net::{TcpListener, TcpStream},
io::prelude::*,
collections::HashMap,
fs,
path::Path,
thread::{self, JoinHandle},
sync::{mpsc, Mutex, Arc}, any, f32::consts::E,
ffi::OsStr
};

fn main() {
    let ip = "0.0.0.0";
    let port = "6969";

    let pool = ThreadPool::new(8);

    let socket =
        TcpListener::bind(format!("{}:{}", ip, port)).expect("unable to read from socket");

    println!("Webserver is running at {}:{}", ip, port);
    println!("");

    for data in socket.incoming() {
        let data = data.expect("unable to read from socket");

        pool.execute(|| {
            handleconnection(data);
        });
    }
}

fn handleconnection(mut data: TcpStream) {
    let mut buffer: [u8;2024] = [0;2024];
    data.read(&mut buffer).expect("unable to read from socket");

    let request_data = String::from_utf8_lossy(&buffer);
    let request = HttpRequest::new(request_data.to_string());

    if request.method == "GET" {
        let htmldir = "./html";

        let filename: String = if request.uri == "/" {
            "index.html".to_string()
        }
        else if request.uri.contains("?v=") {
            request.uri.split("?v=").take(1).collect()
        }
        else if Path::new(&request.uri).extension().unwrap_or_default().to_string_lossy() == "" {
            format!("{}/index.html", request.uri)
        }
        else {
            request.uri
        };

        let path: String = format!("{}/{}", htmldir, filename);

        if Path::new(&path).exists() {

            let mime_type =
                Path::new(&path).extension().unwrap_or_default().to_string_lossy();
            
                let mime_type = if mime_type == "js" {
                    "javascript".to_string()
                } else if mime_type == "" {
                    "html".to_string()
                } else {
                    mime_type.to_string()
                };
            
            let content = fs::read(path).unwrap();

            let content_type = format!("text/{}", mime_type);
            let content_length: usize = content.len();
            let status_line = "HTTP/1.1 200 OK";

            let response = format!(
                "{status_line}\r\nContent-Length: {content_length}\r\nContent-Type: {content_type}\r\n\r\n",
                status_line = status_line,
                content_length = content_length,
                content_type = content_type,
            );

            data.write(response.as_bytes()).expect("unable to write response data");
            data.write(&content).expect("unable to write response data");
            data.flush().expect("unable to flush response data");
        } else {
            let responsedir = "./responses";
            let filename = "404.html";
            let path: String = format!("{}/{}", responsedir, filename);

            let mime_type =
                Path::new(&path).extension().expect("unable to read extension").to_string_lossy();

                let mime_type = if mime_type == "js" {
                    "javascript".to_string()
                } else {
                    mime_type.to_string()
                };

            let content = fs::read(path).unwrap();

            let content_type = format!("text/{}", mime_type);
            let content_length: usize = content.len();
            let status_line = "HTTP/1.1 404 NOT FOUND";

            let response = format!(
                "{status_line}\r\nContent-Length: {content_length}\r\nContent-Type: {content_type}\r\n\r\n",
                status_line = status_line,
                content_length = content_length,
                content_type = content_type,
            );
            data.write(response.as_bytes()).expect("unable to write response data");
            data.write(&content).expect("unable to write response data");
            data.flush().expect("unable to flush response data");
        }

    } else {
            let responsedir = "./responses";
            let filename = "500.html";
            let path: String = format!("{}/{}", responsedir, filename);

            let mime_type =
                Path::new(&path).extension().expect("unable to read extension").to_string_lossy();

                let mime_type = if mime_type == "js" {
                    "javascript".to_string()
                } else {
                    mime_type.to_string()
                };

            let content = fs::read(path).unwrap();

            let content_type = format!("text/{}", mime_type);
            let content_length: usize = content.len();
            let status_line = "HTTP/1.1 500 Internal Server Error";

            let response = format!(
                "{status_line}\r\nContent-Length: {content_length}\r\nContent-Type: {content_type}\r\n\r\n",
                status_line = status_line,
                content_length = content_length,
                content_type = content_type,
            );
            data.write(response.as_bytes()).expect("unable to write response data");
            data.write(&content).expect("unable to write response data");
            data.flush().expect("unable to flush response data");
    };

    println!("Request: {}", String::from_utf8_lossy(&buffer[..])
    );
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    senders: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (senders, receiver) =
            mpsc::channel();

        let receiver =
            Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(
                id, 
                Arc::clone(&receiver)
            ));
        }

        ThreadPool { workers, senders }
}

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send +'static {
            let job = Box::new(f);
            self.senders.send(job).unwrap();
        }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread =
            thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing", id);

                job()
            });

        Worker { id, thread }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    method: String,
    uri: String,
    version: String,
    headers: HashMap<String, String>,
    body: String
}

impl HttpRequest {
    pub fn new(request_data: String) -> Self {
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