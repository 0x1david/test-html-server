    use std::net::TcpListener;
    use std::io::Read;

pub struct Server {
        address: String,
    }

fn arr(a: &[u8; 5]) {

}

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
                           stream.read(&mut buffer); 
                    }
                    Err(e) => println!("Failed to establish a connection: {}", e),
                }
            }
        }
    }
