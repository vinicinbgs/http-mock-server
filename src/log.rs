use std::{
    io::{self, Write}
};
use chrono::{DateTime, Utc};

#[path = "./utils/util_strings.rs"]
mod util_strings;

pub struct Log {
    pub ip: String,
}

impl Log {
    pub fn emit(&self, data: &str) {
        let stderr = io::stderr();
        let mut handle = stderr.lock();
    
        let now: DateTime<Utc> = Utc::now();
        let mut context = data.to_string().replace("\n", "");
        
        util_strings::remove_whitespace(&mut context);
    
        write!(handle, "{ip} - [{now}] - {context}\n", ip = self.ip).unwrap();
    }   
}