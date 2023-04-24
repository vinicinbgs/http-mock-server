use std::fs;
use serde_json::{json, Value::{Null, self}};

pub fn execute(http_path: &str, http_method: &str, http_request_body: String) -> Value {
    let file_string = fs::read_to_string("./mock_data.json")
        .expect("Unable to read file");

        let mut data: serde_json::Value = serde_json::from_str(&file_string).expect("Unable to parse");

        let data_request_body = data[http_path][http_method]["$.request"].to_owned();

        if data[http_path][http_method] == Null  {
            return json!({
                "error": "URI Path or HTTP Method Not found",
                "path": http_path,
                "method": http_method,
            });
        }

        if data_request_body.to_string() != http_request_body.to_string() {
            let request: Value = serde_json::from_str(&http_request_body).unwrap_or_default();
            return json!({
                "error": "Request body does not match",
                "request": request,
            });
        }

        let _ = &data[http_path][http_method].as_object_mut().unwrap().remove("$.request");

        return data[http_path][http_method].to_owned();
}