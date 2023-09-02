use http::Request;
use http::HTTPMethod;
use server::Server;

mod server;
mod http;
fn main() {
    let get = HTTPMethod::GET;
    let delete = HTTPMethod::DELETE;
    let post = HTTPMethod::POST;
    let put = HTTPMethod::PUT;
    let string = String::from("127.0.0.1:8080");
    let server = Server::new(string);

    server.run();
}
