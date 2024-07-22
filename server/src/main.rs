use tokio::net::UdpSocket;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(addr).await?;
    println!("Server listening on: {}", addr);

    let mut buf = [0; 1024];
    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {}", len, src);

        let response = format!("Echo: {}", String::from_utf8_lossy(&buf[..len]));
        socket.send_to(response.as_bytes(), &src).await?;
    }
}