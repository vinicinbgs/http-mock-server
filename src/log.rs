use chrono::{DateTime, Utc};
use std::io::{self, Write};

#[path = "./utils/util_strings.rs"]
mod util_strings;

pub struct HttpLog {
    pub ip: String,
    pub http_method_path: String,
}

impl HttpLog {
    pub fn emit(&self, data: &str) {
        let stderr = io::stderr();
        let mut handle = stderr.lock();

        let now: DateTime<Utc> = Utc::now();
        let mut context = data.to_string().replace("\n", "");

        util_strings::remove_whitespace(&mut context);

        write!(
            handle,
            "{ip} - [{now}] - {http_method_path} - {context}\n",
            ip = self.ip,
            http_method_path = self.http_method_path
        )
        .unwrap();
    }
}
