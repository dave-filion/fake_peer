use tokio::net::TcpListener;
use tokio::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    // convert listener into incoming connection stream
    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(mut socket) => {

                    println!("accepted connection from {:?}", socket.peer_addr());
                    // spawn future that handles conversation
                    tokio::spawn(async move {

                        let (mut reader, mut writer) = socket.split();

                        // listen to reader, response to writer

                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => {
                                println!("Wrote {} bytes", amt);
                            },
                            Err(e) => {
                                eprintln!("IO error {:?}", e);
                            }
                        }
                    });
                },
                Err(e) => {
                    println!("accept error = {:?}", e);
                }
            }
        }
    };

    println!("server starting on {}", addr);

    // start the server and block
    server.await;
}
