use std::fs;

mod lists;

#[derive(Debug, serde::Deserialize)]
struct RawGameConfig {
    field: Option<RawFieldInfo>,
    lists: Option<Vec<RawListInfo>>,
    conditions: Option<Vec<RawConditionsInfo>>,
    events: Option<Vec<RawEventsInfo>>,
    tiles: Option<Vec<RawTilesInfo>>,
}

#[derive(Debug, serde::Deserialize)]
struct RawFieldInfo {
    title: String,
    base_dice: String,
    help_info: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct RawListInfo {
    name: String,
    elements: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct RawConditionsInfo {
    id: usize,
    rule: String,
}

#[derive(Debug, serde::Deserialize)]
struct RawEventsInfo {
    state: String,
    condition_id: usize,
}

#[derive(Debug, serde::Deserialize)]
struct RawTilesInfo {
    pos: usize,
    title: String,
    description: String,
    rules: Vec<String>,
    color: String,
    condition_id: usize,
}

// pub struct GameConfig {
//     lists: Vec<ListInfo>
// }


fn create_config_entry(game_config: &RawGameConfig) {

}

pub fn read_config_field(field_path: String) {
    let config_raw_text = fs::read_to_string(field_path).unwrap();
    let config: RawGameConfig = toml::from_str(&config_raw_text).unwrap();

    println!("{config:?}");
}
