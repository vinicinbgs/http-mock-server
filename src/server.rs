use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    str,
};

#[path = "./log.rs"]
mod http_log;
use http_log::HttpLog;

pub struct HttpFields {
    pub body: String,
    pub url: String,
    pub method: String,
}

pub fn start() -> TcpListener {
    let dns: &str = "0.0.0.0";
    let port: &str = ":7878";
    let tcp: String = dns.to_owned() + &port.to_owned();

    return TcpListener::bind(tcp).unwrap();
}

pub fn request(mut stream: &TcpStream) -> HttpFields {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let data = str::from_utf8(&buffer).unwrap();

    let ip = stream.local_addr().unwrap();

    let request_vec: Vec<_> = data.split("\r\n").collect();
    let mut skip_line: bool = false;
    let mut line_count: i32 = 0;
    let mut body: &str = "";
    let mut http_method_path: &str = "";

    for line in &request_vec {
        line_count += 1;

        if line_count == 1 {
            http_method_path = line;
        }

        if line.is_empty() {
            skip_line = true;
            continue;
        }

        if skip_line {
            body = line.trim_end_matches("\0");
        }
    }

    let access_log = HttpLog {
        ip: ip.to_string(),
        http_method_path: http_method_path.to_string(),
    };

    access_log.emit(body);

    return HttpFields {
        body: body.to_string(),
        url: url(http_method_path),
        method: method(http_method_path),
    };
}

fn url(http_method_path: &str) -> String {
    let splitted: Vec<&str> = http_method_path.split(" ").collect();

    return String::from(splitted[1]);
}

fn method(http_method_path: &str) -> String {
    let splitted: Vec<&str> = http_method_path.split(" ").collect();

    return String::from(splitted[0]);
}
