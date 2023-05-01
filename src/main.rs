mod server;

#[path = "./controllers/mock_controller.rs"]
mod mock_controller;

fn main() {
    let listener = server::start();

    println!("Server started on port {}", server::port());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let http_fields = server::request(&stream);

        mock_controller::mock(stream, http_fields);
    }
}
