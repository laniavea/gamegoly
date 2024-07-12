use std::fs;

use nanoserde::DeJson;

#[derive(Debug, Clone, DeJson)]
struct SerdePlayerData {
    player_location: i32,
}

#[derive(Debug, Clone)]
pub struct PlayerData {
    location: i32,
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
