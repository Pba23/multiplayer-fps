// use tokio::net::UdpSocket;

// pub async fn start_server() {
//     let socket = UdpSocket::bind("0.0.0.0:8080").await.unwrap();
//     println!("Server listening on port 8080");

//     let mut buf = [0; 1024];
//     loop {
//         let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
//         let msg = String::from_utf8(buf[..len].to_vec()).unwrap();
//         println!("Received: '{}' from {}", msg, addr);

//         // Handle message and update game state
//     }
// }

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