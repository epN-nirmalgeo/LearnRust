use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self)  {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((stream, client_addr)) => {
                    println!("Accepted connection from client : {}", client_addr);
                    println!("Byte stream from client : {:?}", &stream);
                },
                Err(err) => {
                    println!("Failed to accept connection. Exception: {}", err);
                },
            }
        }

    }
}    


