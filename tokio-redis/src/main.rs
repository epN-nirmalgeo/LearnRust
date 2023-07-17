use tokio::{
    net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt, AsyncReadExt}
};

mod resp;


#[tokio::main]
async fn main() {
    println!("Starting tokio redis !");
    let listener = TcpListener::bind("localhost:6379").await.unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];

            loop {
                let n = socket.read(&mut buffer).await.expect("failed to read data from socket");

                let str_msg: &str = match std::str::from_utf8(&buffer) {
                    Ok(v) => v,
                    Err(e) => return,
                };

                let response = "*1\r\n$4\r\nPONG\r\n";

                println!("message: {:?}", str_msg);
                
                socket
                    .write_all(response.as_bytes())
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
