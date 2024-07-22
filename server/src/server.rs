use tokio::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn run(socket: UdpSocket) {
    let socket = Arc::new(socket);
    let socket = Arc::clone(&socket);
    
    loop {
        let mut buf = [0; 1024];
        let (len, addr) = socket.recv_from(&mut buf).await.expect("Failed to receive data");

        let received_data = &buf[..len];
        println!("Received data from {}: {:?}", addr, received_data);

        // Echo the received data back to the sender
        socket.send_to(received_data, &addr).await.expect("Failed to send data");
    }
}
