use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use std::sync::Arc;
use std::net::SocketAddr;

mod server;
mod game_state;

#[tokio::main]
async fn main() {
    // Define the server address
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();

    // Create a UDP socket
    let socket = UdpSocket::bind(addr).await.expect("Could not bind socket");

    // Log server start
    println!("Server running on {}", addr);

    // Run the server
    server::run(socket).await;
}
