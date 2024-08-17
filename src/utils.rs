use rand::Rng;
use slint::{Model, VecModel};

use crate::{DiceRoll, ListData, SpecialDice};
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

pub fn roll_dices(dices: slint::ModelRc<DiceRoll>) -> Vec<i32> {
    let dices = dices.as_any().downcast_ref::<VecModel<DiceRoll>>().unwrap();
    let mut dice_rolls = Vec::with_capacity(dices.row_count());

    let mut rng = rand::thread_rng();
    for dice in dices.iter() {
        dice_rolls.push(rng.gen_range(dice.first_bound..dice.second_bound+1));
    }

    dice_rolls
}

pub fn roll_element_from_list(list: ListData) -> slint::SharedString {
    let elements_count = list.list_elements.row_count();

    let mut rng = rand::thread_rng();
    let rolled_element = rng.gen_range(0..elements_count);

    let all_rolls = list.list_elements.as_any().downcast_ref::<VecModel<slint::SharedString>>().unwrap();

    all_rolls.row_data(rolled_element).unwrap().clone()
}
