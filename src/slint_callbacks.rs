use crate::{AppWindow, FieldAdapter, LowerPanelAdapter, InfoPanelAdapter, utils};
use crate::config_player::serialize_player;

use slint::{Model, VecModel};
use slint::Weak;
use slint::ComponentHandle;
pub fn lower_panel_callbacks(window: Weak<AppWindow>) {
    let main_window = window.unwrap();

    // +1 turn
    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_update_player_state(move |player_loc| {
        let new_main_window = main_window_weak.unwrap();
        let field_adapter = new_main_window.global::<FieldAdapter>();

        update_player_pos(&field_adapter, player_loc);
    });

    // Dice roll
    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_roll_dice(move || {
        let new_main_window = main_window_weak.unwrap();
        let field_adapter = new_main_window.global::<FieldAdapter>();
        let info_panel_adapter = new_main_window.global::<InfoPanelAdapter>();

        let dices = utils::roll_dices(field_adapter.get_base_dice());

        let mut dices_sum: i32 = 0;
        let mut dices_max_value: u32 = 0;
        let mut dices_max_value_is_pos: bool = true;

        for value in &dices {
            dices_sum += value;

            if value.unsigned_abs() > dices_max_value {
                dices_max_value = value.unsigned_abs();
                dices_max_value_is_pos = *value >= 0;
            }
        }

        let max_digits = (dices_max_value.checked_ilog10().unwrap_or(0) as i32) + 1;

        info_panel_adapter.set_dices_count(dices.len() as i32);
        info_panel_adapter.set_dices_max_digits(max_digits + if dices_max_value_is_pos {0} else {1});
        info_panel_adapter.set_dices(slint::ModelRc::new(slint::VecModel::from(dices.clone())));
        info_panel_adapter.set_panel_mode(2);

        let new_player_loc= dices_sum + field_adapter.get_player_loc_id();

        update_player_pos(&field_adapter, new_player_loc);

        let special_dices = field_adapter.get_special_dices();

        let condions_ids = utils::special_dices_check(&dices, special_dices);
        if !condions_ids.is_empty() {
            field_adapter.set_conditions_queue(slint::ModelRc::new(slint::VecModel::from(condions_ids)));
        }

        //TODO: TEST, REMOVE THIS 
        let conditions = field_adapter.get_conditions_queue();
        match conditions.as_any().downcast_ref::<VecModel<i32>>() {
            Some(test) => {
                for testik in test.iter() {
                    println!("{:?}", testik);
                }
            },
            None => println!("NO CONDITIONs"),
        }
    });

    // Save player state
    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_save_player_state(move || {
        let new_main_window = main_window_weak.unwrap();
        let field_adapter = new_main_window.global::<FieldAdapter>();

        //TODO:Handle this error
        serialize_player(field_adapter.get_player_loc_id()).unwrap();
    });
}

pub fn field_callbacks(window: Weak<AppWindow>) {
    let main_window = window.unwrap();

    // Roll random element from list
    main_window.global::<FieldAdapter>().on_roll_list_item(move |list_data| {
        utils::roll_element_from_list(list_data)
    });
}


fn update_player_pos(field_adapter: &FieldAdapter, player_loc: i32) {
    let number_of_tiles = field_adapter.get_number_of_tiles();
    let new_player_loc = player_loc % number_of_tiles;

    let (ver_state, hor_state) = utils::get_ver_hor_state(new_player_loc, number_of_tiles);

    field_adapter.set_player_on_ver(ver_state);
    field_adapter.set_player_on_hor(hor_state);
    field_adapter.set_player_loc_id(new_player_loc);
}
