use crate::server::HttpFields;
use regex::Regex;
use serde_json::Value::{self, Null};
use std::{io::Write, net::TcpStream};

#[path = "../services/mock.rs"]
mod mock_service;
use mock_service::{File, Http};

pub fn mock(stream: TcpStream, http_fields: HttpFields) {
    let mut status: &str = "HTTP/1.1 200 OK";

    let http_path = http_fields.original_url.as_str();
    let http_method = http_fields.method.as_str();

    // todo: Improve using dynamic route params
    //let first_path = path.split("/").collect::<Vec<&str>>()[1];
    // let re = Regex::new(format!("/{first_path}/([A-Z|a-z|0-9]*)").as_str()).unwrap();
    // let path_formatted = re.replace_all(path, "/register/:id");
    //println!("{:#}", re.is_match(Some(data).unwrap().to_string().as_str()));

    // Remove \n and spaces from request body
    let re = Regex::new(r"\n\s*|\s").unwrap();
    let http_request_body = re.replace_all(&http_fields.body, "").to_string();
    let ret = mock_service::execute(
        Http {
            path: http_path,
            method: http_method,
            request_body: http_request_body,
        },
        File {
            file_path: String::new(),
        },
    );

    if ret["path"] != Null {
        status = "HTTP/1.1 404 NOT FOUND";
    }

    if ret["request"] != Null {
        status = "HTTP/1.1 400 BAD REQUEST";
    }

    return response(&stream, ret, status);
}

fn response_format(status: String, length: String, content: String) -> String {
    let content_type: &str = "Content-Type: application/json";

    return format!("{status}\r\n{length}\r\n{content_type}\r\n\r\n{content}");
}

fn response(mut stream: &TcpStream, data: Value, status: &str) {
    let content = format!("{}", data);
    let length = format!("Content-Length: {}", content.len());

    stream
        .write_all(response_format(status.to_string(), length, content).as_bytes())
        .unwrap();
}
