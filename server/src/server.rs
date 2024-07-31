use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use std::net::{SocketAddr};
use std::time::{Duration, Instant };
use tokio::time::timeout;
use std::io;
use tokio::net::UdpSocket;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Player {
    id: u32,
    position: (u32 , u32),
    addr: SocketAddr
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameState {
    players: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum PlayerInput {
    Move { id: u32, direction: (u32, u32) },
}

#[derive(Debug)]
pub struct Server {
    pub  socket : UdpSocket,
    pub  clients: Vec<Player>,
    timer : Instant
}

#[derive(Debug)]
struct Client {
    addr: SocketAddr,
    tx: mpsc::Sender<GameState>,
    rx: mpsc::Receiver<PlayerInput>,
}

impl Server {
    pub async fn new() -> Self {
        let socket = UdpSocket::bind("0.0.0.0:8080").await;
        Self {
            socket : socket.unwrap(),
            clients: Vec::new(),
            timer : Instant::now()
        }
    }
    pub async fn accept(&mut self)  {
        let mut buf = [0; 1024];
        loop {
            // println!("wait");
    
            // Timeout de 30 secondes pour l'appel à recv_from
            let recv_result = timeout(Duration::from_secs(1), self.socket.recv_from(&mut buf)).await;
    
            match recv_result {
                Ok(Ok((len, addr))) => {
                    println!("receive");
                    let msg = String::from_utf8_lossy(&buf[..len]);
                    println!("Received from {}: {}", addr, msg);
    
                    let new_player = Player {
                        id: self.clients.len() as u32 + 1,
                        position: (0, 0),
                        addr,
                    };
                    self.clients.push(new_player.clone());
    
                    // Broadcast the message to all clients
                    for client in self.clients.iter() {
                        if client.addr != new_player.addr {
                            self.socket
                                .send_to(&buf[..len], &client.addr)
                                .await
                                .expect("Failed to send data");
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Failed to receive data: {:?}", e);
                }
                Err(_) => {
                    // println!("Timeout after 1 seconds of waiting");
                    if self.clients.len() < 2 {
                        self.timer = Instant::now()
                    } else if self.timer.elapsed() > Duration::from_secs(30) {
                        println!("finish");
                        break;
                    }
                }
            }
    
            // println!("clients: {:?}", self.clients);
        }
    }


    // async fn run(&mut self) {
    //     loop {
    //         //let mut clients = self.clients.clone();
    //         for client in &mut self.clients {
    //             while let Ok(input) = client.rx.try_recv() {
    //                 client.handle_input(input).await;
    //             }
    //         }
    //         self.broadcast_state().await;
    //         tokio::time::sleep(tokio::time::Duration::from_millis(16)).await; // Run at ~60 FPS
    //     }
    // }

    // async fn handle_input(&mut self, input: PlayerInput) {
    //     match input {
    //         PlayerInput::Move { id, direction } => {
    //             if let Some(player) = self.state.players.iter_mut().find(|p| p.id == id) {
    //                 // player.position += direction;
    //             }
    //         }
    //     }
    // }

    // async fn broadcast_state(&self) {
    //     for client in &self.clients {
    //         client.tx.send(self.state.clone()).await.unwrap();
    //     }
    // }
}

// impl Client {
//     async fn handle_input(&mut self, input: PlayerInput) {
//         match input {
//             PlayerInput::Move { id, direction } => {
//                     self.position += direction;
//                 }
//         }
//     }
// }
