use tokio::net::UdpSocket;

pub async fn start_server() {
    let socket = UdpSocket::bind("0.0.0.0:8080").await.unwrap();
    println!("Server listening on port 8080");

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
        let msg = String::from_utf8(buf[..len].to_vec()).unwrap();
        println!("Received: '{}' from {}", msg, addr);

        // Handle message and update game state
    }
}
