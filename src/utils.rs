use rand::Rng;
use slint::{Model, VecModel};

use crate::FieldAdapter;
use crate::{DiceRoll, SpecialDice, FieldTilesData};
use crate::config_field::GameGolyConfigError;

// Returns id of corners for the field
pub fn get_corners(number_of_elems: usize) ->(usize, usize, usize) {
    let main_value = (number_of_elems - 4) / 4;

    (main_value + 1, main_value * 2 + 2, main_value * 3 + 3)
}

// Returns is player on vertical line or on horizontal line
pub fn get_ver_hor_state(player_id: i32, number_of_elems: i32) -> (bool, bool) {
    let elems_in_row = number_of_elems / 4;

    let mut ver_state = player_id >= elems_in_row * 2 && player_id <= elems_in_row * 3;
    ver_state = ver_state || player_id <= elems_in_row;

    let mut hor_state = player_id >= elems_in_row && player_id <= elems_in_row * 2;
    hor_state = hor_state || player_id >= elems_in_row * 3 || player_id == 0;

    (ver_state, hor_state)
}

// Returs vec of DiceRolls based on string, must be used only within config read
pub fn dices_from_string(dice_roll_string: &str) -> Result<Vec<DiceRoll>, Box<dyn std::error::Error>> {
    let mut dice_rolls: Vec<DiceRoll> = vec![];
    for now_roll in dice_roll_string.split_whitespace() {
        let Some(comma_pos) = now_roll.find(',') else {
            return Err(Box::new(GameGolyConfigError::DiceRollNoSeparator))
        };

        if comma_pos == now_roll.len() - 1 {
            return Err(Box::new(GameGolyConfigError::DiceRollIncomplete));
        }

        let first_bound: i32 = now_roll[..comma_pos].parse()?;
        let second_bound: i32 = now_roll[comma_pos+1..].parse()?;

        if second_bound < first_bound {
            return Err(Box::new(GameGolyConfigError::DiceRollIncorrect));
        }

        dice_rolls.push(DiceRoll{
            first_bound,
            second_bound,
        })
    }
    Ok(dice_rolls)
}

// Returns Vec of condition ids if special dice
pub fn special_dices_check(dice_roll: &[i32], special_dices: slint::ModelRc<SpecialDice>) -> Vec<i32>{
    let special_dices = special_dices.as_any().downcast_ref::<VecModel<SpecialDice>>().unwrap();
    let mut condition_ids = vec![];

    for special_dice in special_dices.iter() {
        let res = special_dice.check_roll(dice_roll);
        if let Some(condition_id) = res { condition_ids.push(condition_id) };
    }

    condition_ids
} 

// Function to generate concatinated string to show it in status
pub fn combine_strings(input_strings: slint::ModelRc<slint::SharedString>) -> slint::SharedString {
    let input_strings = input_strings.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap();

    let mut result_string: String = String::from("");

    for now_string in input_strings.iter() {
        result_string.push_str(&format!("{}, ", &now_string));
    }

    if !result_string.is_empty() {
        result_string = result_string[..result_string.len()-2].to_string();
    } else {
        result_string.push_str("None");
    }

    slint::SharedString::from(&result_string)
}

// Dice roll
pub fn roll_dices(dices: slint::ModelRc<DiceRoll>) -> Vec<i32> {
    let dices = dices.as_any().downcast_ref::<VecModel<DiceRoll>>().unwrap();
    let mut dice_rolls = Vec::with_capacity(dices.row_count());

    let mut rng = rand::thread_rng();
    for dice in dices.iter() {
        dice_rolls.push(rng.gen_range(dice.first_bound..dice.second_bound+1));
    }

    dice_rolls
}

// Returns tile data based on player_id because tile's data splitted on 4 parts
pub fn get_tile_data_from_tile_id(tile_num: usize, field_adapter: &FieldAdapter) -> FieldTilesData {
    let number_of_tiles = field_adapter.get_number_of_tiles() as usize;
    let (ul, ur, dr) = get_corners(number_of_tiles);

    if tile_num <= ul {
        let now_tiles = field_adapter.get_field_left();
        let now_tiles = now_tiles.as_any().downcast_ref::<VecModel<FieldTilesData>>().unwrap();
        now_tiles.row_data(ul - tile_num).unwrap()
    } else if tile_num < ur {
        let now_tiles = field_adapter.get_field_top();
        let now_tiles = now_tiles.as_any().downcast_ref::<VecModel<FieldTilesData>>().unwrap();
        now_tiles.row_data(tile_num - ul - 1).unwrap()
    } else if tile_num <= dr {
        let now_tiles = field_adapter.get_field_right();
        let now_tiles = now_tiles.as_any().downcast_ref::<VecModel<FieldTilesData>>().unwrap();
        now_tiles.row_data(tile_num - ur).unwrap()
    } else {
        let now_tiles = field_adapter.get_field_bottom();
        let now_tiles = now_tiles.as_any().downcast_ref::<VecModel<FieldTilesData>>().unwrap();
        now_tiles.row_data(number_of_tiles - tile_num - 1).unwrap()
    }
}

pub fn parse_vec_shared_str(
        strings: slint::ModelRc<slint::SharedString>,
        offset: usize)
    -> Result<Vec<i32>, std::num::ParseIntError> {
    let input_strings = strings.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap();
    let mut digits = vec![];

    for (now_id, now_string) in input_strings.iter().enumerate() {
        if now_id >= offset { break; }
        digits.push(now_string.parse()?)
    }
    Ok(digits)
}

pub fn roll_id_by_number_cummul(input_nums: &[i32]) -> usize {
    let number_sum = input_nums.iter().sum();

    let mut rng = rand::thread_rng();
    let rolled_num = rng.gen_range(0..number_sum);

    let mut temp_sum = 0;
    for (now_id, now_num) in input_nums.iter().enumerate() {
        temp_sum += *now_num;
        if temp_sum > rolled_num {
            return now_id
        }
    }
    unreachable!();
}
