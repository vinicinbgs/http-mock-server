use std::{io::Write, net::TcpStream};

#[path = "../database/db.rs"]
mod db;

pub fn index(mut stream: TcpStream, data: String) {
    let _ = db::create_table();
    let _ = db::insert(&data);

    let status = "HTTP/1.1 200 OK";
    let content = format!(r#"{}"#, data);
    let content_type = "Content-Type: application/json";
    let length = format!("Content-Length: {}", content.len());
    let response = format!("{status}\r\n{length}\r\n{content_type}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
