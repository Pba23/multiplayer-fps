use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use std::time::{Duration, Instant };
use tokio::time::timeout;
use tokio::net::UdpSocket;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub position: (u32 , u32),
    pub addr: SocketAddr,
    pub username : String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameState {
    players: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum PlayerInput {
    Move { id: u32, direction: (u32, u32) },
}
pub type Client = Player;

#[derive(Debug)]
pub struct Server {
    pub  socket : UdpSocket,
    pub  clients: Vec<Client>,
    timer : Instant
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    action : String,
    level : Option<u32>,
    players : Option<Vec<Player>>
}

impl Message {
    fn new(action : String , level : Option<u32> , players : Option<Vec<Player>> ) -> Self {
        Self { action, level, players }
    }
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
    
            // Timeout de 1 secondes pour l'appel à recv_from
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
                        username : msg.to_string()
                    };
                    self.clients.push(new_player.clone());
                }
                Ok(Err(e)) => {
                    eprintln!("Failed to receive data: {:?}", e);
                }
                Err(_) => {
                    // println!("Timeout after 1 seconds of waiting");
                    if self.clients.len() < 2 {
                        self.timer = Instant::now()
                    } else if self.timer.elapsed() > Duration::from_secs(10) {
                        println!("finish");
                        self.broadcast(Message::new("start".to_string(), Some(1), Some(self.clients.clone()))).await;
                        break;
                    }
                }
            }
    
            // println!("clients: {:?}", self.clients);
        }
    }
    pub async fn listen(&self)  {
        let mut buf = [0; 1024];
        loop {
            let (c, addr) = self.socket.recv_from(&mut buf).await.unwrap();
            
            let c : Vec<&Client> = self.clients.iter().filter(|c| c.addr == addr).collect();
            if let Some(c) = c.first() {
                println!("receive message from {:?}",  c);
            }

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

    async fn broadcast(&self , mes : Message) {
         // Broadcast the message to all clients
         let json_data = serde_json::to_string(&mes).unwrap();
         for client in self.clients.iter() {
                self.socket
                    .send_to(json_data.as_bytes(), &client.addr)
                    .await
                    .expect("Failed to send data");
        }
    }
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
