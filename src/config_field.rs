use std::fs;

use crate::FieldData;
use crate::utils;

use nanoserde::DeJson;

#[derive(Debug, Clone, DeJson)]
struct SerdeFieldData {
	title: String,
	description: String,
	rules: Vec<String>,
	condition_id: i32,
}

#[derive(Debug, Clone, DeJson)]
struct SerdeMainInfoData {
    title: String,
}

#[derive(Debug, Clone, DeJson)]
struct SerdeGameGolyData {
    main_info: SerdeMainInfoData,
    field: Vec<SerdeFieldData>,
}

impl SerdeGameGolyData {
    fn field_to_slint(&mut self) -> (
        slint::ModelRc<FieldData>,
        slint::ModelRc<FieldData>,
        slint::ModelRc<FieldData>,
        slint::ModelRc<FieldData>,
        i32
        ) {
        let mut slint_field: Vec<FieldData> = Vec::with_capacity(self.field.len());

        for now_field in self.field.drain(..) {
            let slint_rules: Vec<slint::SharedString> = now_field.rules.iter().map(slint::SharedString::from).collect();

            slint_field.push(
                FieldData{
                    title: slint::SharedString::from(now_field.title),
                    description: slint::SharedString::from(now_field.description),
                    rules: slint::ModelRc::new(slint::VecModel::from(slint_rules)),
                    condition_id: now_field.condition_id,
                    fill_color: slint::Color::from_rgb_u8(127, 127, 127),
            });
        }

        let number_of_tiles: i32 = slint_field.len() as i32;

        let (u_l, u_r, b_r) = utils::get_corners(number_of_tiles as usize);

        let bottom_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(b_r+1..).rev().collect::<Vec<FieldData>>()));

        let right_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(u_r..).collect::<Vec<FieldData>>()));

        let top_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(u_l+1..).collect::<Vec<FieldData>>()));

        let left_part = slint::ModelRc::new(
            slint::VecModel::from(slint_field.drain(..).rev().collect::<Vec<FieldData>>()));

        (
            bottom_part,
            right_part,
            top_part,
            left_part,
            number_of_tiles,
        )
    }

    fn main_info_to_slint(&mut self) -> slint::SharedString {
        slint::SharedString::from(self.main_info.title.clone())
    }
}

pub struct GameGolyDataSlint {
    field_number_of_elems: i32,
    field_top: slint::ModelRc<FieldData>,
    field_left: slint::ModelRc<FieldData>,
    field_right: slint::ModelRc<FieldData>,
    field_bottom: slint::ModelRc<FieldData>,
    main_info_title: slint::SharedString,
}

impl GameGolyDataSlint {
    pub fn field_number_of_elems(&self) -> i32 {
        self.field_number_of_elems
    }

    pub fn field_top(&self) -> slint::ModelRc<FieldData> {
        self.field_top.clone()
    }

    pub fn field_left(&self) -> slint::ModelRc<FieldData> {
        self.field_left.clone()
    }

    pub fn field_right(&self) -> slint::ModelRc<FieldData> {
        self.field_right.clone()
    }

    pub fn field_bottom(&self) -> slint::ModelRc<FieldData> {
        self.field_bottom.clone()
    }
    
    pub fn main_info_title(&self) -> slint::SharedString {
        self.main_info_title.clone()

    }
}

pub fn read_config(file_path: &str) -> Result<GameGolyDataSlint, Box<dyn std::error::Error>>{
    let config_text = fs::read_to_string(file_path)?;
    let mut gamegoly_data: SerdeGameGolyData = DeJson::deserialize_json(&config_text)?;

    let (field_slint_bottom,
        field_slint_right,
        field_slint_top,
        field_slint_left,
        number_of_tiles
    ) = gamegoly_data.field_to_slint();

    let main_info_title = gamegoly_data.main_info_to_slint();

    let gamegoly_data_slint = GameGolyDataSlint {
        field_number_of_elems: number_of_tiles,
        field_top: field_slint_top,
        field_left: field_slint_left,
        field_right: field_slint_right,
        field_bottom: field_slint_bottom,
        main_info_title,
    };

    Ok(gamegoly_data_slint)
}
