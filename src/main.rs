mod server;

#[path = "./controllers/home_controller.rs"]
mod home_controller;

fn main() {
    let listener = server::start();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let http_fields = server::request(&stream);

        match http_fields.method.as_str() {
            "GET" => {
                home_controller::index(stream, http_fields.body);
            }
            _ => print!("HEY"),
        }
    }
}
