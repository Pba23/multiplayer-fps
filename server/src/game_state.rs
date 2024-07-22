use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: u32,
    pub username: String,
    pub position: (f32, f32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub players: Vec<Player>,
}

impl GameState {
    pub fn new() -> Self {
        GameState { players: Vec::new() }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player_id: u32) {
        self.players.retain(|player| player.id != player_id);
    }

    pub fn update_player_position(&mut self, player_id: u32, position: (f32, f32)) {
        if let Some(player) = self.players.iter_mut().find(|p| p.id == player_id) {
            player.position = position;
        }
    }
}
