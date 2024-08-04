use crate::DiceRoll;

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

pub fn dices_from_string(dice_roll_string: String) -> Vec<DiceRoll> {
    let mut dice_rolls: Vec<DiceRoll> = vec![];
    //TODO: Handle some errors in comma_pos, parse ints
    for now_roll in dice_roll_string.split_whitespace() {
        let comma_pos = now_roll.find(',').unwrap();

        if comma_pos == now_roll.len() - 1 {
            unimplemented!("Handle error when , is last character")
        }

        let first_bound: i32 = now_roll[..comma_pos].parse().unwrap();
        let second_bound: i32 = now_roll[comma_pos+1..].parse().unwrap();

        if second_bound <= first_bound {
            unimplemented!("Handle error when bounds equal or not ok")
        }

        dice_rolls.push(DiceRoll{
            first_bound,
            second_bound,
        })
    }
    dice_rolls
}

//TODO: Remove or recreate using RollDice structs if so
pub fn _roll_next_dice(_dice_rolls_str: &mut str) -> Vec<i32> {
    vec![]
}
