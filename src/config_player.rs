use std::fs;

use slint::{Model, VecModel};
use nanoserde::{DeJson, SerJson};

use crate::LowerPanelAdapter;

#[derive(Debug, Clone, DeJson, SerJson)]
struct SerdePlayerData {
    player_location: i32,
    player_state: i32,
    specials: Vec<String>,
    add_tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PlayerDataSlint {
    location: i32,
    state: i32,
    specials: slint::ModelRc<slint::SharedString>,
    add_tags: slint::ModelRc<slint::SharedString>
}

impl PlayerDataSlint {
    pub fn location(&self) -> i32 {
        self.location
    }

    pub fn state(&self) -> i32 {
        self.state
    }

    pub fn specials(&self) -> slint::ModelRc<slint::SharedString> {
        self.specials.clone()
    }

    pub fn add_tags(&self) -> slint::ModelRc<slint::SharedString> {
        self.add_tags.clone()
    }
}

pub fn read_config(file_path: &str) -> Result<PlayerDataSlint, Box<dyn std::error::Error>> {
    let config_text = fs::read_to_string(file_path)?;
    let player_data: SerdePlayerData = DeJson::deserialize_json(&config_text)?;

    let mut specials: Vec<slint::SharedString> = vec![];
    for special in player_data.specials {
        specials.push(slint::SharedString::from(&special));
    }

    let mut add_tags: Vec<slint::SharedString> = vec![];
    for add_tag in player_data.add_tags {
        add_tags.push(slint::SharedString::from(&add_tag));
    }

    Ok(PlayerDataSlint {
        location: player_data.player_location,
        state: player_data.player_state,
        specials: slint::ModelRc::new(slint::VecModel::from(specials)),
        add_tags: slint::ModelRc::new(slint::VecModel::from(add_tags)),
    })
}

pub fn serialize_player(pl_loc: i32, lower_panel_adapter: LowerPanelAdapter) -> Result<(), Box<dyn std::error::Error>> {

    let specials = lower_panel_adapter.get_player_special();
    let add_tags = lower_panel_adapter.get_player_add_tags();

    let new_special = specials.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap();
    let new_add_tags = add_tags.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap();

    let end_specials: Vec<String> = new_special.iter().map(|now_st| now_st.into()).collect();
    let end_add_tags: Vec<String> = new_add_tags.iter().map(|now_st| now_st.into()).collect();

    let serde_player_data = SerdePlayerData {
        player_location: pl_loc,
        player_state: lower_panel_adapter.get_player_status(),
        specials: end_specials,
        add_tags: end_add_tags,
    };

    let result = SerJson::serialize_json(&serde_player_data);
    fs::write("player.json", result)?;
    Ok(())
}
