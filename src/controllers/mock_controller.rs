use std::{io::Write, net::TcpStream};

use crate::server::HttpFields;

#[path = "../database/db.rs"]
mod db;

const STATUS: &str = "HTTP/1.1 200 OK";
const CONTENT_TYPE: &str = "Content-Type: application/json";

pub fn store(mut stream: TcpStream, http_fields: HttpFields) {
    let _ = db::create_table();
    let _ = db::insert(&http_fields.original_url, &http_fields.body);

    let content = format!(r#"{}"#, &http_fields.body);
    let length = format!("Content-Length: {}", content.len());

    stream
        .write_all(response_format(length, content).as_bytes())
        .unwrap();
}

pub fn index(mut stream: TcpStream, http_fields: HttpFields) {
    let body = db::get_mock_path(&http_fields.original_url);

    let content = format!("{}", body.unwrap());
    let length = format!("Content-Length: {}", content.len());

    stream
        .write_all(response_format(length, content).as_bytes())
        .unwrap();
}

fn response_format(length: String, content: String) -> String {
    return format!("{STATUS}\r\n{length}\r\n{CONTENT_TYPE}\r\n\r\n{content}");
}
