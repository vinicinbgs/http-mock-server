use crate::server::HttpFields;
use serde_json::Value::{self, Null};
use std::{io::Write, net::TcpStream};

#[path = "../services/mock.rs"]
mod mock_service;
use mock_service::{File, Http};

macro_rules! status {
    ($a: expr, $b:expr) => {
        match $a {
            "200" => "HTTP/1.1 200 OK",
            "201" => "HTTP/1.1 201 CREATED",
            "202" => "HTTP/1.1 202 ACCEPTED",
            "203" => "HTTP/1.1 203 NON-AUTHORITATIVE INFORMATION",
            "204" => "HTTP/1.1 204 NO CONTENT",
            "205" => "HTTP/1.1 205 RESET CONTENT",
            "206" => "HTTP/1.1 206 PARTIAL CONTENT",
            "300" => "HTTP/1.1 300 MULTIPLE CHOICES",
            "301" => "HTTP/1.1 301 MOVED PERMANENTLY",
            "302" => "HTTP/1.1 302 FOUND",
            "303" => "HTTP/1.1 303 SEE OTHER",
            "304" => "HTTP/1.1 304 NOT MODIFIED",
            "305" => "HTTP/1.1 305 USE PROXY",
            "307" => "HTTP/1.1 307 TEMPORARY REDIRECT",
            "308" => "HTTP/1.1 308 PERMANENT REDIRECT",
            "400" => "HTTP/1.1 400 BAD REQUEST",
            "401" => "HTTP/1.1 401 UNAUTHORIZED",
            "402" => "HTTP/1.1 402 PAYMENT REQUIRED",
            "403" => "HTTP/1.1 403 FORBIDDEN",
            "404" => "HTTP/1.1 404 NOT FOUND",
            "405" => "HTTP/1.1 405 METHOD NOT ALLOWED",
            "406" => "HTTP/1.1 406 NOT ACCEPTABLE",
            "407" => "HTTP/1.1 407 PROXY AUTHENTICATION REQUIRED",
            "408" => "HTTP/1.1 408 REQUEST TIMEOUT",
            "409" => "HTTP/1.1 409 CONFLICT",
            "410" => "HTTP/1.1 410 GONE",
            "411" => "HTTP/1.1 411 LENGTH REQUIRED",
            "412" => "HTTP/1.1 412 PRECONDITION FAILED",
            "413" => "HTTP/1.1 413 PAYLOAD TOO LARGE",
            "414" => "HTTP/1.1 414 URI TOO LONG",
            "415" => "HTTP/1.1 415 UNSUPPORTED MEDIA TYPE",
            "416" => "HTTP/1.1 416 RANGE NOT SATISFIABLE",
            "417" => "HTTP/1.1 417 EXPECTATION FAILED",
            "418" => "HTTP/1.1 418 I'M A TEAPOT",
            "421" => "HTTP/1.1 421 MISDIRECTED REQUEST",
            "422" => "HTTP/1.1 422 UNPROCESSABLE ENTITY",
            "423" => "HTTP/1.1 423 LOCKED",
            "424" => "HTTP/1.1 424 FAILED DEPENDENCY",
            "425" => "HTTP/1.1 425 TOO EARLY",
            "426" => "HTTP/1.1 426 UPGRADE REQUIRED",
            "428" => "HTTP/1.1 428 PRECONDITION REQUIRED",
            "429" => "HTTP/1.1 429 TOO MANY REQUESTS",
            "431" => "HTTP/1.1 431 REQUEST HEADER FIELDS TOO LARGE",
            "451" => "HTTP/1.1 451 UNAVAILABLE FOR LEGAL REASONS",
            "500" => "HTTP/1.1 500 INTERNAL SERVER ERROR",
            "501" => "HTTP/1.1 501 NOT IMPLEMENTED",
            "502" => "HTTP/1.1 502 BAD GATEWAY",
            "503" => "HTTP/1.1 503 SERVICE UNAVAILABLE",
            "504" => "HTTP/1.1 504 GATEWAY TIMEOUT",
            "505" => "HTTP/1.1 505 HTTP VERSION NOT SUPPORTED",
            "506" => "HTTP/1.1 506 VARIANT ALSO NEGOTIATES",
            "507" => "HTTP/1.1 507 INSUFFICIENT STORAGE",
            "508" => "HTTP/1.1 508 LOOP DETECTED",
            "510" => "HTTP/1.1 510 NOT EXTENDED",
            "511" => "HTTP/1.1 511 NETWORK AUTHENTICATION REQUIRED",
            _ => $b,
        }
    };
}

pub fn mock(stream: TcpStream, http_fields: HttpFields) {
    let mut status: &str = "HTTP/1.1 200 OK";

    let http_path = http_fields.original_url.as_str();
    let http_method = http_fields.method.as_str();

    // todo: Improve using dynamic route params
    //let first_path = path.split("/").collect::<Vec<&str>>()[1];
    // let re = Regex::new(format!("/{first_path}/([A-Z|a-z|0-9]*)").as_str()).unwrap();
    // let path_formatted = re.replace_all(path, "/register/:id");
    //println!("{:#}", re.is_match(Some(data).unwrap().to_string().as_str()));

    let ret = mock_service::execute(
        Http {
            path: http_path,
            method: http_method,
            request_body: http_fields.body,
        },
        File {
            file_path: String::new(),
        },
    );

    if ret["$.body"]["path"] != Null {
        status = "HTTP/1.1 404 NOT FOUND";
    }

    if ret["$.body"]["request"] != Null {
        status = "HTTP/1.1 400 BAD REQUEST";
    }

    return response(
        &stream,
        ret["$.body"].to_owned(),
        status!(ret["$.status"].to_string().as_str(), status),
    );
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
