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
                Ok((mut stream, client_addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                       Ok(_) => {
                         println!("Received buffer: {}", String::from_utf8_lossy(&buffer))
                       },
                       Err(_) => {
                        
                       },
                    }
                    
                },
                Err(err) => {
                    println!("Failed to accept connection. Exception: {}", err);
                },
            }
        }

    }
}    


