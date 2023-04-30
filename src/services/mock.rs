use serde_json::{
    json,
    Value::{self, Null},
};

use std::{env, fs};

pub fn execute(http_path: &str, http_method: &str, http_request_body: String) -> Value {
    let args: Vec<String> = env::args().collect();
    let binding = String::from("./mock_data.json");
    let file_path = args.get(1).unwrap_or(&binding);
    let file_string = fs::read_to_string(file_path)
        .expect(format!("\x1b[31m<< Unable to read file {file_path} >>\x1b[0m").as_str());

    let mut data: serde_json::Value = serde_json::from_str(&file_string).expect("Unable to parse");

    let data_request_body = data[http_path][http_method]["$.request"].to_owned();

    if data[http_path][http_method] == Null {
        return json!({
            "error": "URI Path or HTTP Method Not found",
            "path": http_path,
            "method": http_method,
        });
    }

    if data_request_body.to_string() != http_request_body.to_string()
        && data_request_body.to_string() != "null"
    {
        let request: Value = serde_json::from_str(&http_request_body).unwrap_or_default();
        return json!({
            "error": "Request body does not match",
            "request": request,
        });
    }

    let _ = &data[http_path][http_method]
        .as_object_mut()
        .unwrap()
        .remove("$.request");

    return data[http_path][http_method].to_owned();
}
