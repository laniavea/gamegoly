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

pub fn _roll_next_dice(_dice_rolls_str: &mut str) -> Vec<i32> {
    vec![]
}
