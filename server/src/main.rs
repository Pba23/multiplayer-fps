mod server;
use server::*;
#[tokio::main]
async fn main() {
    let mut  server = Server::new().await;
    
    server.accept().await;
    
    server.listen().await;
    
    println!("hello")

}
// 127.0.0.1:8080