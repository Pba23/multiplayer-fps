use bevy::prelude::*;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use std::io::{self, Write};

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

// Implement the Resource trait for ServerDetails
#[derive(Resource)]
struct ServerDetails {
    ip_address: String,
    username: String,
    state_rx: mpsc::Receiver<GameState>,
    input_tx: mpsc::Sender<PlayerInput>,
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
    let username = prompt("Enter your username: ");
    let ip_address = prompt("Enter server IP address: ");

    // Initialize the state_rx and input_tx channels
    let (state_tx, state_rx) = mpsc::channel(32);
    let (input_tx, input_rx) = mpsc::channel(32);

    // Initialize the Bevy application
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ServerDetails {
            ip_address,
            username,
            state_rx,
            input_tx,
        })
        .add_state::<LocalGameState>()
        .add_startup_system(setup)
        .add_system(update_state)
        .add_system(handle_input.run_if(in_state(LocalGameState::Playing)))
        .run();
}

// Prompt function to capture user input from the terminal
fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Setup function to initialize the Bevy window
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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