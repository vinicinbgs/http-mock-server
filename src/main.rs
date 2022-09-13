mod server;

fn main() {
    let listener = server::start();
   
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let res = server::request(&stream);

        server::response(stream, res);
    }
}