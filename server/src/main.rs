use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8080").await.expect("Could not bind socket");
    println!("Server listening on port 8080");

    // Store the addresses of connected clients
    let clients = Arc::new(Mutex::new(HashMap::new()));

    let mut buf = [0; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await.expect("Failed to receive data");
        let msg = String::from_utf8_lossy(&buf[..len]);

        println!("Received from {}: {}", addr, msg);

        let mut clients_guard = clients.lock().await;
        clients_guard.insert(addr, msg.to_string());

        // Broadcast the message to all clients
        for (&client_addr, _) in clients_guard.iter() {
            if client_addr != addr {
                socket.send_to(&buf[..len], &client_addr).await.expect("Failed to send data");
            }
        }
    }
}
//127.0.0.1:8080