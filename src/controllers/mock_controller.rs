use std::{io::Write, net::TcpStream};
use serde_json::Value::{Null, self};
use regex::Regex;
use crate::server::HttpFields;

#[path = "../services/mock.rs"]
mod mock_service;

pub fn mock(stream: TcpStream, http_fields: HttpFields) {
    let mut status: &str = "HTTP/1.1 200 OK";

    // let file_string = fs::read_to_string("./mock_data.json")
    //     .expect("Unable to read file");
    //let mut data: serde_json::Value = serde_json::from_str(&file_string).expect("Unable to parse");
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
    let ret = mock_service::execute(http_path, http_method, http_request_body);
    
    if ret["path"] != Null {
        status = "HTTP/1.1 404 NOT FOUND";
    }

    if ret["request"] != Null {
        status = "HTTP/1.1 400 BAD REQUEST";
    }

    // let data_request_body = data[http_path][http_method]["$.request"].to_owned();

    // // Check Path / Method in JSON and Return 404 NOT FOUND
    // if data[http_path][http_method] == Null  {
    //     status = "HTTP/1.1 404 NOT FOUND";
    //     data = json!({
    //         "error": "URI Path or HTTP Method Not found"
    //     });
    //     return response(&stream, data, status);
    // }

    // // Check Request Body in JSON and Return 400 BAD REQUEST
    // if data_request_body.to_string() != http_request_body.to_string() {
    //     status = "HTTP/1.1 400 BAD REQUEST";
    //     data = json!({
    //         "error": "Request body does not match"
    //     });
    //     return response(&stream, data, status);
    // }

    // // Remove $.request Body from JSON
    // let _ = &data[http_path][http_method].as_object_mut().unwrap().remove("$.request");
    
    return response(&stream, ret, status)

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