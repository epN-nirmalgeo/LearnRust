fn main() {
    let address = String::from("127.0.0.1:8080");
    let server = Server::new(address);
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Server {
            addr
        }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}