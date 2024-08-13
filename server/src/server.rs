use serde::{Serialize, Deserialize};
use serde_json::from_str;
use std::net::SocketAddr;
use std::time::{Duration, Instant };
use tokio::time::timeout;
use tokio::net::UdpSocket;
use clap::Parser;




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub position: Option<Vec3>,
    pub addr: SocketAddr,
    pub username : String,
    pub lives : u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameState {
    players: Vec<Player>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
     #[arg(short, long, default_value = "1")]
    level: Option<u32>,
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
    players : Option<Vec<Player>>,
    curr_player : Option<Player>,
    position : Option<Vec3>,
    senderid : Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  Vec3 {
    x : f32 , 
    y : f32,
    z : f32
}
impl Message {
    fn new(action : String , level : Option<u32> , players : Option<Vec<Player>> , position : Option<Vec3> ) -> Self {
        Self { action, level,  players , position , curr_player : None , senderid : None }
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
        let args = Args::parse();

        if let Some(level) = args.level {
            println!("Level: {}", level);
        } else {
            println!("No level specified");
        }
        let mut buf = [0; 1024];
        loop {
            // Timeout de 1 secondes pour l'appel à recv_from
            let recv_result = timeout(Duration::from_secs(1), self.socket.recv_from(&mut buf)).await;
    
            match recv_result {
                Ok(Ok((len, addr))) => {
                    println!("receive");
                    let msg = String::from_utf8_lossy(&buf[..len]);
                    println!("Received from {}: {}", addr, msg);
    
                    let new_player = Player {
                        id: self.clients.len() as u32 + 1, 
                        position: None,
                        addr,
                        username : msg.to_string(),
                        lives : 3
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
                        self.broadcast(Message::new("start".to_string(), args.level, Some(self.clients.clone()) ,  Some(Vec3 { x: 0.0, y: 0.0, z: 0.0 }))).await;
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
            
            let cl : Vec<&Client> = self.clients.iter().filter(|c| c.addr == addr).collect();
            if let Some(_pl) = cl.first() {
                let msg = String::from_utf8_lossy(&buf[..c]);

                
                self.broadcast_str(msg.to_string()).await;
            }
 
        }
    }

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
    async fn broadcast_str(&self , mes : String) {
        // Broadcast the message to all clients
        let mess : Message = from_str(&mes).unwrap();
        let sender_id = mess.senderid.unwrap();
        for client in self.clients.iter() {
            if client.id == sender_id {
                continue;
            }
            self.socket
                .send_to(mes.as_bytes(), &client.addr)
                .await
                .expect("Failed to send data");
       }
    }
    
}
