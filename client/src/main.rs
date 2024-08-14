use bevy::prelude::*;
pub use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::math;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use std::{
    io::{self, Write}, net::{SocketAddr, UdpSocket}, sync::{Arc, Mutex}
};
pub mod game;

use game::{connexion::{listen, update_ressources}, interface_in_2d::*};
use game::fps_display::*;
use game::interface_in_3d::*;
use game::laser::*;

// Structs
// #[derive(Component)]

#[derive(Debug, Clone, Serialize, Deserialize )]
pub struct Player {
    pub id: u32,
    pub position: Option<Vec3>,
    pub addr: SocketAddr,
    pub username : String,
    rotation : Option<Quat>,
    pub lives  : u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub players: Vec<Player>,
    pub playing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum PlayerInput {
    Move { id: u32, direction: Vec2 },
}

// Implement the Resource trait for ServerDetails
#[derive(Resource, Debug)]
pub struct ServerDetails {
    pub ip_address: String,
    pub username: String,
    // state_rx: mpsc::Receiver<GameState>,
    // input_tx: mpsc::Sender<PlayerInput>,
    pub socket: UdpSocket,
    pub mess: Message,
}

// Define the states for the game
#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum LocalGameState {
    #[default]
    Connecting,
    Playing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  Message {
    action : String,
    level : Option<u32>,
    players : Option<Vec<Player>>,
    curr_player : Option<Player>,
    position : Option<Vec3>,
    senderid : Option<u32>,
    rotation : Option<Quat>
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  Vec3 {
    x : f32 , 
    y : f32,
    z : f32
}
impl Vec3 {
    pub fn to_v3(&self) -> math::Vec3 {
        math::Vec3::new(self.x , self.y , self.z)
    }
    pub fn from_v3(x : f32 , y : f32 , z : f32) -> Self {
        Self{x , y  , z}
    }
}

#[derive(Resource , Debug  )]

pub struct MyChannel {
   pub tx: UnboundedSender<String>,
   pub rx: Arc<Mutex<UnboundedReceiver<String>>>,
}

fn main() {
    // Capture username and IP address from the terminal
    let username = prompt("Enter your username: ");
    let ip_address = prompt("Enter server IP address: ");

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap(); // "0" signifie que le système choisit un port libre
    let mes = username.as_bytes();
    socket
        .send_to(mes, ip_address.clone())
        .expect("failed to connect");

    println!("Waiting for the game to start");
    let mut buf = [0; 1024];
    let mut mess = Message{action : String::new() , level : None , players : None , curr_player : None , position : None , senderid : None , rotation : None};
    println!("message {:?}" , mess);

    loop {
        let (c, _addr) = socket.recv_from(&mut buf).unwrap();
        println!("ADDRESS => {:?}", socket.local_addr());
        let  msg = String::from_utf8_lossy(&buf[..c]).to_string();
        mess = from_str(&msg).expect("ERROR");
        println!("reveived mes : {:?}", mess);
        if mess.action == "start" {
            mess.curr_player =
                getcurrplayer(mess.clone(), socket.local_addr().unwrap().to_string());
            break;
        }
    }
    let (tx, rx) = unbounded_channel();
    let  channel = MyChannel{tx : tx.clone()  , rx : Arc::new(Mutex::new(rx))};
    let channel_clone = MyChannel { 
        tx: channel.tx.clone(), 
        rx: Arc::clone(&channel.rx) 
    };

    listen(socket.try_clone().unwrap() , channel);   

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ServerDetails {
            ip_address,
            username,
            socket,
            mess,
        })
        .insert_resource(channel_clone)
        .add_startup_system(setup)
        .add_startup_system(setup_radar)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_fps_counter)
        .add_system(update_fps_text)
        .add_system(shoot)
        .add_system(player_shoot.run_if(player_is_alive))
        .add_system(update_ressources)
        .add_system(update_laser_positions)
        .add_system(check_laser_collisions)
        .add_system(player_movement.run_if(player_is_alive))
        .add_system(update_position)
        .add_system(camera_follow_player)
        .add_system(update_radar) 
        .add_system(delete_dead_players)
        .add_system(display_game_over.run_if(player_is_dead))
        .run();
}

fn getcurrplayer(m: Message, s: String) -> Option<Player> {
    let id_current = s.split(":").last()?;
    let c = m.players.unwrap();

    for pl in c {
        if pl.addr.to_string().ends_with(id_current) {
            return Some(pl);
        }
    }
    None
}
fn player_is_dead(server_details: Res<ServerDetails>) -> bool {
    !player_is_alive(server_details)
}
fn player_is_alive(server_details: Res<ServerDetails>) -> bool {
    if let Some(curr_player) = &server_details.mess.curr_player {
        if let Some(players) = &server_details.mess.players {
            if let Some(player) = players.iter().find(|p| p.id == curr_player.id) {
                return player.lives > 0;
            }
        }
    }
    false
}

// Prompt function to capture user input from the terminal
fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

