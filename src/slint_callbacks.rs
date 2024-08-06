use crate::{AppWindow, FieldAdapter, LowerPanelAdapter, utils};

use slint::Weak;
use slint::ComponentHandle;

pub fn lower_panel_callbacks(window: Weak<AppWindow>){
    let main_window = window.unwrap();

    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_update_player_state(move |player_loc| {
        let new_main_window = main_window_weak.unwrap();
        let field_adapter = new_main_window.global::<FieldAdapter>();

        update_player_pos(field_adapter, player_loc);
    });

    let main_window_weak = main_window.as_weak();
    main_window.global::<LowerPanelAdapter>().on_roll_dice(move || {
        let new_main_window = main_window_weak.unwrap();
        let field_adapter = new_main_window.global::<FieldAdapter>();

        let dices = utils::roll_dices(field_adapter.get_base_dice());
        let new_player_loc= dices.iter().sum::<i32>() + field_adapter.get_player_loc_id();

        update_player_pos(field_adapter, new_player_loc);
    });
}

fn update_player_pos(field_adapter: FieldAdapter, player_loc: i32) {
    let number_of_tiles = field_adapter.get_number_of_tiles();
    let new_player_loc = player_loc % number_of_tiles;

    let (ver_state, hor_state) = utils::get_ver_hor_state(new_player_loc, number_of_tiles);

    field_adapter.set_player_on_ver(ver_state);
    field_adapter.set_player_on_hor(hor_state);
    field_adapter.set_player_loc_id(new_player_loc);
}
