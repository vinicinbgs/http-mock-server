use serde_json::{
    json,
    Value::{self, Null},
};

use std::{
    env,
    fs::{self, File},
    io::Read,
};

trait DataFile {
    fn get_path(&self) -> String;
}

pub struct MockFile {
    pub file_path: String,
}

impl DataFile for MockFile {
    fn get_path(&self) -> String {
        let args: Vec<String> = env::args().collect();

        let file = String::from(if self.file_path != "" {
            self.file_path.to_string()
        } else {
            "mock_data.json".to_string()
        });

        let mut i = 0;

        let file_path = loop {
            let arg = args.get(i).unwrap_or(&file);

            if arg.len() > 2 && &arg[..3] == "-f=" {
                break arg[3..].to_string();
            }

            if arg == &file {
                break file.to_string();
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

pub enum TypeOr<S, T> {
    Left(S),
    Right(T),
}

pub fn execute(http: Http, file: MockFile) -> TypeOr<Vec<u8>, Value> {
    let file_path = file.get_path();

    let file_string = fs::read_to_string(&file_path)
        .expect(format!("\x1b[31m<< Unable to read file {file_path} >>\x1b[0m").as_str());

    let mut data: Value = serde_json::from_str(&file_string).expect("Unable to parse");

    let data_request_body = data[http.path][http.method]["$.request"].to_owned();

    if data[http.path][http.method] == Null {
        return TypeOr::Right(json!({
            "$.body": {
                "error": "URI Path or HTTP Method Not found",
                "path": http.path,
                "method": http.method,
            },
            "$.status": "404",
        }));
    }

    if check_http_request_body_is_different_from_data_request_body(
        data_request_body.to_string(),
        http.request_body.to_string(),
    ) || check_data_request_body_is_null_and_http_request_body_is_not_empty(
        data_request_body.to_string(),
        http.request_body.to_string(),
    ) {
        let request: Value = serde_json::from_str(&http.request_body).unwrap_or_default();
        return TypeOr::Right(json!({
            "$.body": {
                "error": "Request body does not match",
                "request": request,
            },
            "$.status": "400",
        }));
    }

    let _ = &data[http.path][http.method]
        .as_object_mut()
        .unwrap()
        .remove("$.request");

    if data[http.path][http.method]["$.response"]["$.file"] != Null {
        let filename = data[http.path][http.method]["$.response"]["$.file"]
            .as_str()
            .unwrap();

        let mut f = match File::open(&filename) {
            Ok(file) => file,
            Err(_) => panic!("Unable to open asset file {}", filename),
        };

        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        return TypeOr::Left(buffer);
    }

    return TypeOr::Right(data[http.path][http.method]["$.response"].to_owned());
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
            MockFile {
                file_path: "src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Left(_) => panic!("Should not return binary data"),
            TypeOr::Right(ret) => {
                assert_eq!(ret["$.body"]["name"], "John Doe");
            }
        }
    }

    #[test]
    fn test_read_file_and_return_mock_not_found() {
        let ret = execute(
            Http {
                path: "/register",
                method: "GET",
                request_body: "".to_string(),
            },
            MockFile {
                file_path: "src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Left(_) => panic!("Should not return binary data"),
            TypeOr::Right(ret) => {
                assert_eq!(ret["$.body"]["error"], "URI Path or HTTP Method Not found");
            }
        }
    }

    #[test]
    fn test_read_file_and_return_mock_request_body_does_not_match() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: r#"{"name": "what_your_name"}"#.to_string(),
            },
            MockFile {
                file_path: "src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Left(_) => panic!("Should not return binary data"),
            TypeOr::Right(ret) => {
                assert_eq!(ret["$.body"]["error"], "Request body does not match");
            }
        }
    }

    #[test]
    fn test_read_file_from_args_and_return_mock_request_body_match() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: "".to_string(),
            },
            MockFile {
                file_path: "-f=src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Left(_) => panic!("Should not return binary data"),
            TypeOr::Right(ret) => {
                assert_eq!(ret["$.body"]["name"], "John Doe");
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_exception_when_input_wrong_file_path() {
        execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: "".to_string(),
            },
            MockFile {
                file_path: "file_that_not_exist.json".to_string(),
            },
        );
    }

    #[test]
    fn test_read_from_default_file_and_return_mock_request_body_match() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: "".to_string(),
            },
            MockFile {
                file_path: "./src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Left(_) => panic!("Should not return binary data"),
            TypeOr::Right(ret) => {
                assert_eq!(ret["$.body"]["name"], "John Doe");
            }
        }
    }
}

// todo: rename file to match_mock
