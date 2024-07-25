use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use bevy::math::Vec2;

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

#[derive(Debug)]
struct Server {
    state: GameState,
    clients: Vec<Client>,
}

#[derive(Debug)]
struct Client {
    addr: SocketAddr,
    tx: mpsc::Sender<GameState>,
    rx: mpsc::Receiver<PlayerInput>,
}

impl Server {
    fn new() -> Self {
        Self {
            state: GameState { players: Vec::new() },
            clients: Vec::new(),
        }
    }

    async fn run(&mut self) {
        loop {
            for client in &mut self.clients {
                while let Ok(input) = client.rx.try_recv() {
                    self.handle_input(input).await;
                }
            }
            self.broadcast_state().await;
            tokio::time::sleep(tokio::time::Duration::from_millis(16)).await; // Run at ~60 FPS
        }
    }

    async fn handle_input(&mut self, input: PlayerInput) {
        match input {
            PlayerInput::Move { id, direction } => {
                if let Some(player) = self.state.players.iter_mut().find(|p| p.id == id) {
                    player.position += direction;
                }
            }
        }
    }

    async fn broadcast_state(&self) {
        for client in &self.clients {
            client.tx.send(self.state.clone()).await.unwrap();
        }
    }
}
