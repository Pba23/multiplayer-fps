use bevy::prelude::*;
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use std::io::{self, Write};

// Structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    id: u32,
    position: Vec2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    players: Vec<Player>,
    level: u32, // Add the level field
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerInput {
    Move { id: u32, direction: Vec2 },
}

// Implement the Resource trait for ServerDetails
#[derive(Resource)]
pub struct ServerDetails {
    ip_address: String,
    username: String,
    state_rx: mpsc::Receiver<GameState>,
    input_tx: mpsc::Sender<PlayerInput>,
}

// Define the states for the game
#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum LocalGameState {
    #[default]
    Connecting,
    Playing,
    LevelUp, // Add the LevelUp state
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
        .add_system(handle_level_up.run_if(in_state(LocalGameState::LevelUp))) // Add the handle_level_up system
        .run();
}

// Prompt function to capture user input from the terminal
pub fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Setup function to initialize the Bevy window
pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// System to update the game state
pub fn update_state(
    mut next_state: ResMut<NextState<LocalGameState>>,
    mut server_details: ResMut<ServerDetails>,
) {
    if let Ok(new_state) = server_details.state_rx.try_recv() {
        if new_state.level > 1 {
            next_state.set(LocalGameState::LevelUp);
        } else {
            next_state.set(LocalGameState::Playing);
        }
    }
}

// System to handle player input
pub fn handle_input(
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

// System to handle level up state
pub fn handle_level_up(
    mut next_state: ResMut<NextState<LocalGameState>>,
    mut server_details: ResMut<ServerDetails>,
) {
    println!("Level up!");

    // Perform level-up logic here, such as displaying a message or updating the game

    // Transition back to the Playing state
    next_state.set(LocalGameState::Playing);
}
