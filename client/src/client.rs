use tokio::net::UdpSocket;
use std::net::SocketAddr;

pub async fn connect_to_server(ip: &str, username: &str) {
    let addr: SocketAddr = format!("{}:8080", ip).parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    println!("Client bound to {}", socket.local_addr().unwrap());
    
    let msg = format!("CONNECT {}", username);
    socket.send_to(msg.as_bytes(), &addr).await.unwrap();
    println!("Sent: '{}' to {}", msg, addr);

    /// Buffer used for storing data.
    let mut buf = [0; 1024];
    let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
    let from_utf8 = String::from_utf8(buf[..len].to_vec()).unwrap();
    let msg = from_utf8;
    println!("Received: '{}' from {}", msg, addr);

    // Handle received message and update client state
    match msg.trim() {
        "START_GAME" => {
            println!("Game started!");
            // TODO: Implement game logic
        }
        "STOP_GAME" => {
            println!("Game stopped!");
            // TODO: Implement game stop logic
        }
        _ => {
            println!("Unknown message received: '{}'", msg);
            // TODO: Handle other message types
        }
    }
}
