use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str
};

#[path = "./log.rs"]
mod log;
use log::Log;

pub fn start() -> TcpListener {
    let dns: &str = "0.0.0.0";
    let port: &str = ":7878";
    let tcp: String = dns.to_owned() + &port.to_owned();

    return TcpListener::bind(tcp).unwrap();
}

pub fn request(mut stream: &TcpStream) -> String {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let data = str::from_utf8(&buffer).unwrap();
    
    let ip = stream.local_addr().unwrap();

    let request_vec: Vec<_> = data.split("\r\n").collect();
    let mut skip_line: i32 = 0;
    let mut body = "";

    for line in &request_vec {
        if line.is_empty() {
            skip_line += 1;
            continue;
        }

        if skip_line == 1 {
            body = line.trim_end_matches("\0");
        }
    }

    let log = Log {
        ip: ip.to_string(),
    };

    log.emit(body);

    return body.to_string();
}

pub fn response(mut stream: TcpStream, data: String) {
    let status = "HTTP/1.1 200 OK";
    let content = format!(r#"{}"#, data);
    let content_type = "Content-Type: application/json";
    let length = format!("Content-Length: {}",content.len());

    let response = format!("{status}\r\n{length}\r\n{content_type}\r\n\r\n{content}");
    
    stream.write_all(response.as_bytes()).unwrap();
}