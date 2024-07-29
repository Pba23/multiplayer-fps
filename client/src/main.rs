use bevy::prelude::*;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use std::io::{self, Write};
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;

// Structs
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Player {
    id: u32,
    position: Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameState {
    players: Vec<Player>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum PlayerInput {
    Move { id: u32, direction: Vec2 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConnectionInfo {
    username: String,
    client_address: String,
}

// Implement the Resource trait for ServerDetails
#[derive(Resource)]
struct ServerDetails {
    ip_address: String,
    username: String,
    state_rx: mpsc::Receiver<GameState>,
    input_tx: mpsc::Sender<PlayerInput>,
    socket: UdpSocket,
    connected: bool,
}

// Define the states for the game
#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum LocalGameState {
    #[default]
    Connecting,
    Playing,
}

// Entry point
fn main() {
    // Capture username and IP address from the terminal
    let ip_address = prompt("Enter server IP address: ");
    let username = prompt("Enter your username: ");

    // Initialize the state_rx and input_tx channels
    let (state_tx, state_rx) = mpsc::channel(32);
    let (input_tx, input_rx) = mpsc::channel(32);

    // Create a Tokio runtime
    let runtime = Runtime::new().unwrap();

    // Bind the UDP socket
    let socket = runtime.block_on(async {
        UdpSocket::bind("0.0.0.0:0").await.expect("Could not bind to address")
    });

    // Setup function to initialize the Bevy window
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Prompt function to capture user input from the terminal
fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


    // Initialize the Bevy application
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ServerDetails {
            ip_address: ip_address.clone(),
            username: username.clone(),
            state_rx,
            input_tx,
            socket,
            connected: false,
        })
        .add_state::<LocalGameState>()
        .add_startup_system(setup)
        .add_system(connect_to_server)
        .add_system(update_state)
        .add_system(handle_input.run_if(in_state(LocalGameState::Playing)))
        .run();
}

// System to connect to the server
fn connect_to_server(mut server_details: ResMut<ServerDetails>) {
    if !server_details.connected {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(async {
            let server_address = format!("{}:12345", server_details.ip_address);
            let client_address = server_details.socket.local_addr().unwrap().to_string();
            
            let connection_info = ConnectionInfo {
                username: server_details.username.clone(),
                client_address,
            };
            
            let serialized = serde_json::to_string(&connection_info).unwrap();
            
            match server_details.socket.send_to(serialized.as_bytes(), &server_address).await {
                Ok(_) => {
                    println!("Connected to server successfully");
                    server_details.connected = true;
                },
                Err(e) => eprintln!("Failed to connect to server: {}", e),
            }
        });
    }
}

// System to update the game state
fn update_state(
    mut next_state: ResMut<NextState<LocalGameState>>,
    mut server_details: ResMut<ServerDetails>,
) {
    if let Ok(new_state) = server_details.state_rx.try_recv() {
        // Update the game state based on the received state
        // This is a placeholder - you'll need to implement the actual state update logic
        next_state.set(LocalGameState::Playing);
    }
}

// System to handle player input
fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut server_details: ResMut<ServerDetails>,
) {
    let direction = if keyboard_input.pressed(KeyCode::W) {
        Vec2::new(0.0, 1.0)
    } else if keyboard_input.pressed(KeyCode::S) {
        Vec2::new(0.0, -1.0)
    } else if keyboard_input.pressed(KeyCode::A) {
        Vec2::new(-1.0, 0.0)
    } else if keyboard_input.pressed(KeyCode::D) {
        Vec2::new(1.0, 0.0)
    } else {
        Vec2::ZERO
    };

    if direction != Vec2::ZERO {
        let input = PlayerInput::Move { id: 1, direction }; // Example player ID
        let _ = server_details.input_tx.try_send(input);
    }
}