use serde_json::{
    json,
    Value::{self, Null},
};

use std::{env, fs};

trait DataFile {
    fn get_path(&self) -> String;
}

pub struct File {
    pub file_path: String,
}

impl DataFile for File {
    fn get_path(&self) -> String {
        let args: Vec<String> = env::args().collect();

        let default = String::from(if self.file_path != "" {
            self.file_path.to_string()
        } else {
            "./mock_data.json".to_string()
        });

        let mut i = 0;

        let file_path = loop {
            let arg = args.get(i).unwrap_or(&default);

            if &arg[..3] == "-f=" {
                break arg[3..].to_string();
            }

            if arg == &default {
                break default.to_string();
            }

            i += 1;
        };

        return file_path;
    }
}

pub struct Http<'a> {
    pub path: &'a str,
    pub method: &'a str,
    pub request_body: String,
}

pub fn execute(http: Http, file: File) -> Value {
    let file_path = file.get_path();

    let file_string = fs::read_to_string(&file_path)
        .expect(format!("\x1b[31m<< Unable to read file {file_path} >>\x1b[0m").as_str());

    let mut data: Value = serde_json::from_str(&file_string).expect("Unable to parse");

    let data_request_body = data[http.path][http.method]["$.request"].to_owned();

    if data[http.path][http.method] == Null {
        return json!({
            "$.body": {
                "error": "URI Path or HTTP Method Not found",
                "path": http.path,
                "method": http.method,
            }
        });
    }

    if check_http_request_body_is_different_from_data_request_body(
        data_request_body.to_string(),
        http.request_body.to_string(),
    ) || check_data_request_body_is_null_and_http_request_body_is_not_empty(
        data_request_body.to_string(),
        http.request_body.to_string(),
    ) {
        let request: Value = serde_json::from_str(&http.request_body).unwrap_or_default();
        return json!({
            "$.body": {
                "error": "Request body does not match",
                "request": request,
            }
        });
    }

    let _ = &data[http.path][http.method]
        .as_object_mut()
        .unwrap()
        .remove("$.request");

    return data[http.path][http.method]["$.response"].to_owned();
}

fn check_http_request_body_is_different_from_data_request_body(
    data_request_body: String,
    http_request_body: String,
) -> bool {
    return data_request_body != http_request_body && data_request_body != "null";
}

fn check_data_request_body_is_null_and_http_request_body_is_not_empty(
    data_request_body: String,
    http_request_body: String,
) -> bool {
    return data_request_body == "null" && http_request_body != "";
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_file_and_return_mock_successfully() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: "".to_string(),
            },
            File {
                file_path: "src/services/test_mock_data.json".to_string(),
            },
        );

        assert_eq!(ret["$.body"]["name"], "John Doe");
    }

    #[test]
    fn test_read_file_and_return_mock_not_found() {
        let ret = execute(
            Http {
                path: "/register",
                method: "GET",
                request_body: "".to_string(),
            },
            File {
                file_path: "src/services/test_mock_data.json".to_string(),
            },
        );

        assert_eq!(ret["$.body"]["error"], "URI Path or HTTP Method Not found");
    }

    #[test]
    fn test_read_file_and_return_mock_request_body_does_not_match() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: r#"{"name": "what_your_name"}"#.to_string(),
            },
            File {
                file_path: "src/services/test_mock_data.json".to_string(),
            },
        );

        assert_eq!(ret["$.body"]["error"], "Request body does not match");
    }

    #[test]
    fn test_read_file_from_args_and_return_mock_request_body_match() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: "".to_string(),
            },
            File {
                file_path: "-f=src/services/test_mock_data.json".to_string(),
            },
        );

        assert_eq!(ret["$.body"]["name"], "John Doe");
    }

    #[test]
    fn test_read_from_default_file_and_return_mock_request_body_match() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: "".to_string(),
            },
            File {
                file_path: "".to_string(),
            },
        );

        assert_eq!(ret["$.body"]["name"], "John Doe");
    }
}
