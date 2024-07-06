use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
#[tokio::main]
async fn main() {

    server();
    loop {client().await.unwrap()}
}

async fn server() -> Result<(), Box<dyn Error + Sync + Send>> {
    println!("Server");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        println!("{socket:?}");
    }
}

async fn client() -> Result<(), Box<dyn Error + Sync + Send>> {
    println!("Client");
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    loop {
        stream.write_all(b"hello world!").await?;
    }
}
