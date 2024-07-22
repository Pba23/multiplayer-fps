use std::collections::HashMap;
use std::net::SocketAddr;

pub struct GameState {
    players: HashMap<SocketAddr, Player>,
    current_level: usize,
}

pub struct Player {
    username: String,
    x: f32,
    y: f32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: HashMap::new(),
            current_level: 1,
        }
    }

    pub fn add_player(&mut self, addr: SocketAddr, username: String) {
        let player = Player {
            username,
            x: 0.0,
            y: 0.0,
        };
        self.players.insert(addr, player);
    }

    pub fn remove_player(&mut self, addr: &SocketAddr) {
        self.players.remove(addr);
    }

    pub fn update_player_position(&mut self, addr: &SocketAddr, x: f32, y: f32) {
        if let Some(player) = self.players.get_mut(addr) {
            player.x = x;
            player.y = y;
        }
    }

    pub fn get_player_positions(&self) -> Vec<(String, f32, f32)> {
        self.players
            .iter()
            .map(|(_, player)| (player.username.clone(), player.x, player.y))
            .collect()
    }

    pub fn advance_level(&mut self) {
        self.current_level += 1;
    }
}