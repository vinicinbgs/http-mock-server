use chrono::{DateTime, Utc};
use serde_json;
use std::io::{self, Write};

pub struct HttpLog {
    pub ip: String,
    pub http_method_path: String,
}

impl HttpLog {
    pub fn emit(&self, data: &str) {
        let stderr = io::stderr();
        let mut handle = stderr.lock();

        let now: DateTime<Utc> = Utc::now();

        let context = serde_json::to_string(data).unwrap();

        write!(
            handle,
            "{ip} - [{now}] - {http_method_path} - {context}\n",
            ip = self.ip,
            http_method_path = self.http_method_path
        )
        .unwrap();
    }
}
