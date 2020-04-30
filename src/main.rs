use tokio::{net::TcpListener, stream::StreamExt};
use fake_peer::print_byte_array_len;
use tokio::net::TcpStream;
use tokio::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();
    while let Some(stream) = listener.next().await {
        match stream {
            Ok(mut stream) => {
                println!("new client addr: {:?}", stream.peer_addr());

                // read handshake
                let mut buf = [0u8; 512];
                let bytes_read = stream.read(&mut buf).await.unwrap();
                println!("Read {} bytes", bytes_read);
                print_byte_array_len("handshake request", &buf, bytes_read);
            }
            Err(e) => { /* connection failed */ }
        }
    }
}