use std::fs;

use crate::FieldTilesData;
use crate::utils;

use nanoserde::DeJson;

#[derive(Debug, Clone, DeJson)]
struct SerdeTileData {
	title: String,
	description: String,
	rules: Vec<String>,
    color: String,
	condition_id: i32,

    #[nserde(default)]
    fill_color: (u8, u8, u8)
}

#[derive(Debug, Clone, DeJson)]
struct SerdeFieldMainData {
    title: String,
    base_roll: String,
}

impl SerdeFieldMainData {
    pub fn base_roll(&self) -> String {
        self.base_roll.clone()
    }
}

#[derive(Debug, Clone, DeJson)]
struct SerdeGameGolyData {
    main_info: SerdeFieldMainData,
    field: Vec<SerdeTileData>,
}

#[derive(Debug, Clone)]
pub enum GameGolyConfigError {
    ColorOverflow,
    IncorrectNumberOfTiles,
}

impl std::fmt::Display for GameGolyConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GameGolyConfigError::ColorOverflow => write!(f, "Color contains more than three elements"),
            GameGolyConfigError::IncorrectNumberOfTiles => write!(f, "Number of field tiles must be devided by 4 without reminder"),
        }
    } }

impl std::error::Error for GameGolyConfigError {}

impl SerdeGameGolyData {
    fn main_info(&self) -> &SerdeFieldMainData {
        &self.main_info
    }
}

impl SerdeGameGolyData {
    fn config_validation(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        SerdeGameGolyData::validate_field(&mut self.field)?;
        Ok(())
    }

    fn validate_field(field: &mut [SerdeTileData]) -> Result<(), Box<dyn std::error::Error>> {
        for tile in field.iter_mut() {
            let rgb = tile.color.split_whitespace();

            let mut temp_color = [0u8; 3];

            for (color_id, color_value) in rgb.into_iter().enumerate() {
                if color_id == 3 {
                    return Err(Box::new(GameGolyConfigError::ColorOverflow))
                }
                temp_color[color_id] = color_value.parse()?;
            }

            tile.fill_color = temp_color.into();
        }

        if field.len() % 4 != 0 || field.is_empty() {
            return Err(Box::new(GameGolyConfigError::IncorrectNumberOfTiles));
        }

        Ok(())
    }
}

impl SerdeGameGolyData {
    fn field_tiles_to_slint(&mut self) -> (
        slint::ModelRc<FieldTilesData>,
        slint::ModelRc<FieldTilesData>,
        slint::ModelRc<FieldTilesData>,
        slint::ModelRc<FieldTilesData>,
        i32
        ) {
        let mut slint_field: Vec<FieldTilesData> = Vec::with_capacity(self.field.len());

        for now_field in self.field.drain(..) {
            let slint_rules: Vec<slint::SharedString> = now_field.rules.iter().map(slint::SharedString::from).collect();

            let (color_r, color_g, color_b) = now_field.fill_color;

            slint_field.push(
                FieldTilesData {
                    title: slint::SharedString::from(now_field.title),
                    description: slint::SharedString::from(now_field.description),
                    rules: slint::ModelRc::new(slint::VecModel::from(slint_rules)),
                    condition_id: now_field.condition_id,
                    fill_color: slint::Color::from_rgb_u8(color_r, color_g, color_b),
            }); }

        let number_of_tiles: i32 = slint_field.len() as i32;

        let (u_l, u_r, b_r) = utils::get_corners(number_of_tiles as usize);

        let bottom_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(b_r+1..).rev().collect::<Vec<FieldTilesData>>()));

        let right_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(u_r..).collect::<Vec<FieldTilesData>>()));

        let top_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(u_l+1..).collect::<Vec<FieldTilesData>>()));

        let left_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(..).rev().collect::<Vec<FieldTilesData>>()));

        (
            bottom_part,
            right_part,
            top_part,
            left_part,
            number_of_tiles,
        )
    }

    fn main_info_to_slint(&mut self) -> (slint::SharedString, i32) {
        (slint::SharedString::from(self.main_info.title.clone()), 0)
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
    _base_roll: String,
}

impl FieldMainDataSlint {
    pub fn main_title(&self) -> slint::SharedString {
        self.main_title.clone()
    }

    pub fn _base_roll(&self) -> &str {
        &self._base_roll
    }
}

pub fn read_config(file_path: &str) -> Result<(FieldTilesDataSlint, FieldMainDataSlint), Box<dyn std::error::Error>> {
    let config_text = fs::read_to_string(file_path)?;
    let mut gamegoly_data: SerdeGameGolyData = DeJson::deserialize_json(&config_text)?;

    gamegoly_data.config_validation()?;

    let (field_slint_bottom,
        field_slint_right,
        field_slint_top,
        field_slint_left,
        number_of_tiles
    ) = gamegoly_data.field_tiles_to_slint();

    let field_data_tiles_slint = FieldTilesDataSlint {
        field_number_of_elems: number_of_tiles,
        field_top: field_slint_top,
        field_left: field_slint_left,
        field_right: field_slint_right,
        field_bottom: field_slint_bottom,
    };

    let (main_title, _) = gamegoly_data.main_info_to_slint();

    let field_data_main_slint = FieldMainDataSlint {
        main_title, 
        _base_roll: gamegoly_data.main_info().base_roll(),
    };

    Ok((field_data_tiles_slint, field_data_main_slint))
}
