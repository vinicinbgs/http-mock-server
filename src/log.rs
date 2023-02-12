use chrono::{DateTime, Utc};
use serde_json;
use std::io::{self, StderrLock, Write};

mod config;
pub struct HttpLog {
    pub ip: String,
    pub http_method_path: String,
}

struct LogFormat<'a> {
    handle: StderrLock<'a>,
    context: String,
    now: DateTime<Utc>,
    http_log: HttpLog,
}

impl HttpLog {
    pub fn emit(&self, data: &str) {
        let stderr = io::stderr();
        let handle = stderr.lock();

        let now: DateTime<Utc> = Utc::now();

        let context = serde_json::to_string(data).unwrap();

        Self::log_format({
            LogFormat {
                handle: handle,
                context: context,
                now: now,
                http_log: {
                    HttpLog {
                        ip: self.ip.to_string(),
                        http_method_path: self.http_method_path.to_string(),
                    }
                },
            }
        });
    }

    fn log_format(fields: LogFormat) {
        let env_configs = config::get();
        let mut handle = fields.handle;
        let ip = fields.http_log.ip;
        let now = fields.now;
        let http_method_path = fields.http_log.http_method_path;
        let context = fields.context;

        if env_configs["LOG_FORMAT"] == "\"csv\"" {
            write!(
                handle,
                "{ip}, {now}, {http_method_path}, {context}\n",
                ip = ip,
                http_method_path = http_method_path
            )
            .unwrap();
        }

        if env_configs["LOG_FORMAT"] == "\"ncsa\"" {
            write!(
                handle,
                "{ip} [{now}] {http_method_path} {context}\n",
                ip = ip,
                http_method_path = http_method_path
            )
            .unwrap();
        }
    }
}
