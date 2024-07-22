use tokio::net::UdpSocket;
use std::net::SocketAddr;

pub async fn connect_to_server(ip: &str, username: &str) {
    let addr: SocketAddr = format!("{}:8080", ip).parse().unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    println!("Client bound to {}", socket.local_addr().unwrap());

    let msg = format!("CONNECT {}", username);
    socket.send_to(msg.as_bytes(), &addr).await.unwrap();
    println!("Sent: '{}' to {}", msg, addr);

    let mut buf = [0; 1024];
    let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
    let msg = String::from_utf8(buf[..len].to_vec()).unwrap();
    println!("Received: '{}' from {}", msg, addr);

    // Handle received message and update client state
}
