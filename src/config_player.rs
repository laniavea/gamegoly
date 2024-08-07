use std::fs;

use nanoserde::{DeJson, SerJson};

#[derive(Debug, Clone, DeJson, SerJson)]
struct SerdePlayerData {
    player_location: i32,
}

#[derive(Debug, Clone)]
pub struct PlayerData {
    pub location: i32,
}

impl PlayerData {
    pub fn location(&self) -> i32 {
        self.location
    }
}

pub fn read_config(file_path: &str) -> Result<PlayerData, Box<dyn std::error::Error>> {
    let config_text = fs::read_to_string(file_path)?;
    let player_data: SerdePlayerData = DeJson::deserialize_json(&config_text)?;

    Ok(PlayerData{
        location: player_data.player_location,
    })
}

pub fn serialize_player(player_loc: i32) -> Result<(), Box<dyn std::error::Error>> {
    let serde_player_data = SerdePlayerData {
        player_location: player_loc,
    };

    let result = SerJson::serialize_json(&serde_player_data);
    fs::write("player.json", result)?;
    Ok(())
}
