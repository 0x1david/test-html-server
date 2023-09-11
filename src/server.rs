use crate::http::{Request, Response, StatusCode};
use std::io::{Write, Read};
use std::net::TcpListener;
use std::convert::TryFrom;

pub struct Server {
    address: String,
}

fn arr(a: &[u8; 5]) {}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Receieved a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    let response = Response::new(StatusCode::Ok, Some("<h1> A Response!<h1>".to_string()));
                                    write!(stream, "{}", response);
                                }
                                Err(e) => println!("Failed"),
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
