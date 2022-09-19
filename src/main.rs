mod server;

#[path = "./controllers/mock_controller.rs"]
mod mock_controller;

fn main() {
    let listener = server::start();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let http_fields = server::request(&stream);

        match http_fields.method.as_str() {
            "POST" => mock_controller::store(stream, http_fields),
            "GET" => mock_controller::index(stream, http_fields),
            _ => println!("_"),
        }
    }
}
