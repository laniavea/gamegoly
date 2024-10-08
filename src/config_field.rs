use std::fs;

use crate::{FieldTilesData, DiceRoll, HelpData, ListData, SpecialDice, Condition};
use crate::utils;

use nanoserde::DeJson;

#[derive(Debug, Clone, DeJson)]
struct SerdeTileData {
	title: String,
	description: String,
	rules: Vec<String>,
    color: String,
	condition_id: i32,
}

#[derive(Debug, Clone, DeJson)]
struct SerdeSpecialDice {
    state: String,
    condition_id: i32,
}

#[derive(Debug, Clone, DeJson)]
struct SerdeFieldMainData {
    title: String,
    base_dice: String,
    help_text_headers: Vec<String>,
    help_text: Vec<String>,
    special_dices: Vec<SerdeSpecialDice>,
}

#[derive(Debug, Clone, DeJson)]
struct SerdeListData {
    name: String,
    elements: Vec<String>,
}

#[derive(Debug, Clone, DeJson)]
struct SerdeConditionData {
    id: i32,
    rule: String,
}

#[derive(Debug, Clone, DeJson)]
struct SerdeGameGolyData {
    main_data: SerdeFieldMainData,
    static_lists: Vec<SerdeListData>,
    conditions: Vec<SerdeConditionData>,
    field: Vec<SerdeTileData>,
}

#[derive(Debug, Clone)]
pub enum GameGolyConfigError {
    ColorOverflow(usize),
    IncorrectNumberOfTiles,
    DiceRollIncomplete,
    DiceRollIncorrect,
    DiceRollNoSeparator,
    IncorrectNumberOfHelpTexts,
}

impl std::fmt::Display for GameGolyConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GameGolyConfigError::ColorOverflow(field_num) =>
                write!(f, "Color in line {field_num} contains more than three elements"),
            GameGolyConfigError::IncorrectNumberOfTiles => 
                write!(f, "Number of field tiles must be devided by 4 without reminder and be greater than 7"),
            GameGolyConfigError::DiceRollIncomplete =>
                write!(f, "Base roll always must contain 2 digits separated by comma"),
            GameGolyConfigError::DiceRollIncorrect => 
                write!(f, "Base roll is a range of random number (min,max), these values can't be reverted"),
            GameGolyConfigError::DiceRollNoSeparator => 
                write!(f, "Base roll must contain two digits separated by comma"),
            GameGolyConfigError::IncorrectNumberOfHelpTexts => 
                write!(f, "For every help button name must be only one help text"),
        }
    } }

impl std::error::Error for GameGolyConfigError {}

impl SerdeGameGolyData {
    fn field_tiles_to_slint(&mut self) -> Result<FieldTilesDataSlint, Box<dyn std::error::Error>> {
        let number_of_tiles: i32 = self.field.len() as i32;

        if number_of_tiles % 4 != 0 || number_of_tiles < 8 {
            eprintln!("Found {number_of_tiles} tiles, exiting");
            return Err(Box::new(GameGolyConfigError::IncorrectNumberOfTiles));
        }

        let mut slint_field: Vec<FieldTilesData> = Vec::with_capacity(number_of_tiles as usize);

        for (now_field_id, now_field) in self.field.drain(..).enumerate() {
            let slint_rules: Vec<slint::SharedString> = now_field.rules.iter().map(slint::SharedString::from).collect();

            let rgb = now_field.color.split_whitespace();
            let mut temp_color = [0u8; 3];

            for (color_id, color_value) in rgb.into_iter().enumerate() {
                if color_id == 3 {
                    return Err(Box::new(GameGolyConfigError::ColorOverflow(now_field_id+1)))
                }
                temp_color[color_id] = color_value.parse()?;
            }

            slint_field.push(
                FieldTilesData {
                    title: slint::SharedString::from(now_field.title),
                    description: slint::SharedString::from(now_field.description),
                    rules: slint::ModelRc::new(slint::VecModel::from(slint_rules)),
                    condition_id: now_field.condition_id,
                    fill_color: slint::Color::from_rgb_u8(temp_color[0], temp_color[1], temp_color[2]),
            }); 
        }

        let (u_l, u_r, b_r) = utils::get_corners(number_of_tiles as usize);

        let bottom_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(b_r+1..).rev().collect::<Vec<FieldTilesData>>()));

        let right_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(u_r..).collect::<Vec<FieldTilesData>>()));

        let top_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(u_l+1..).collect::<Vec<FieldTilesData>>()));

        let left_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(..).rev().collect::<Vec<FieldTilesData>>()));

        Ok(FieldTilesDataSlint {
            field_number_of_elems: number_of_tiles,
            field_top: top_part,
            field_left: left_part,
            field_right: right_part,
            field_bottom: bottom_part,
        })
    }

    fn main_data_to_slint(&mut self) -> Result<FieldMainDataSlint, Box<dyn std::error::Error>> {
        let main_data = &self.main_data;
        let static_lists = &self.static_lists;
        let base_dice = utils::dices_from_string(&main_data.base_dice)?;

        if main_data.help_text_headers.len() != main_data.help_text.len() {
            return Err(Box::new(GameGolyConfigError::IncorrectNumberOfHelpTexts));
        }

        let mut help_data: Vec<HelpData> = vec![];
        for (now_id, now_header) in main_data.help_text_headers.iter().enumerate() {
            help_data.push(HelpData {
                help_header: slint::SharedString::from(now_header),
                help_text: slint::SharedString::from(&main_data.help_text[now_id]),
            })
        }

        let mut conditions: Vec<Condition> = vec![];
        for condition in &self.conditions {
            conditions.push(Condition {
                condition_id: condition.id,
                rule: slint::SharedString::from(&condition.rule),
            })
        }

        let mut lists: Vec<ListData> = vec![];

        for list in static_lists {
            let mut now_list_elements: Vec<slint::SharedString> = vec![];
            for list_element in &list.elements {
                now_list_elements.push(slint::SharedString::from(list_element))
            }

            lists.push(ListData {
                list_name: slint::SharedString::from(&list.name),
                list_elements: slint::ModelRc::new(slint::VecModel::from(now_list_elements)),
            })
        }

        let mut special_dices: Vec<SpecialDice> = vec![];

        for dice in &main_data.special_dices {
            special_dices.push(SpecialDice{
                state: slint::SharedString::from(&dice.state),
                condition_id: dice.condition_id,
            });
        }

        Ok(FieldMainDataSlint {
            main_title: slint::SharedString::from(&main_data.title), 
            base_dice: slint::ModelRc::new(slint::VecModel::from(base_dice)),
            help_data: slint::ModelRc::new(slint::VecModel::from(help_data)),
            conditions: slint::ModelRc::new(slint::VecModel::from(conditions)),
            static_lists: slint::ModelRc::new(slint::VecModel::from(lists)),
            special_dices: slint::ModelRc::new(slint::VecModel::from(special_dices)),
        })
    }
}

pub struct FieldTilesDataSlint {
    field_number_of_elems: i32,
    field_top: slint::ModelRc<FieldTilesData>,
    field_left: slint::ModelRc<FieldTilesData>,
    field_right: slint::ModelRc<FieldTilesData>,
    field_bottom: slint::ModelRc<FieldTilesData>,
}

impl FieldTilesDataSlint {
    pub fn field_number_of_elems(&self) -> i32 {
        self.field_number_of_elems
    }

    pub fn field_top(&self) -> slint::ModelRc<FieldTilesData> {
        self.field_top.clone()
    }

    pub fn field_left(&self) -> slint::ModelRc<FieldTilesData> {
        self.field_left.clone()
    }

    pub fn field_right(&self) -> slint::ModelRc<FieldTilesData> {
        self.field_right.clone()
    }

    pub fn field_bottom(&self) -> slint::ModelRc<FieldTilesData> {
        self.field_bottom.clone()
    }
}

pub struct FieldMainDataSlint {
    main_title: slint::SharedString,
    base_dice: slint::ModelRc<DiceRoll>,
    help_data: slint::ModelRc<HelpData>,
    conditions: slint::ModelRc<Condition>,
    static_lists: slint::ModelRc<ListData>,
    special_dices: slint::ModelRc<SpecialDice>,
}

impl FieldMainDataSlint {
    pub fn main_title(&self) -> slint::SharedString {
        self.main_title.clone()
    }

    pub fn base_dice(&self) -> slint::ModelRc<DiceRoll> {
        self.base_dice.clone()
    }

    pub fn help_data(&self) -> slint::ModelRc<HelpData> {
        self.help_data.clone()
    }

    pub fn conditions(&self) -> slint::ModelRc<Condition> {
        self.conditions.clone()
    }

    pub fn static_lists(&self) -> slint::ModelRc<ListData> {
        self.static_lists.clone()
    }

    pub fn special_dices(&self) -> slint::ModelRc<SpecialDice> {
        self.special_dices.clone()
    }
}

pub fn read_config(file_path: &str) -> Result<(FieldTilesDataSlint, FieldMainDataSlint), Box<dyn std::error::Error>> {
    let config_text = fs::read_to_string(file_path)?;
    let mut gamegoly_data: SerdeGameGolyData = DeJson::deserialize_json(&config_text)?;

    let field_data_tiles_slint = gamegoly_data.field_tiles_to_slint()?;

    let field_data_main_slint = gamegoly_data.main_data_to_slint()?;

    Ok((field_data_tiles_slint, field_data_main_slint))
}
