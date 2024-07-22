use crate::{AppWindow, FieldAdapter, LowerPanelAdapter, utils};

use slint::Weak;
use slint::ComponentHandle;

pub fn lower_panel_callbacks(window: Weak<AppWindow>){
    let main_window = window.unwrap();
    let main_window_weak = main_window.as_weak();

    main_window.global::<LowerPanelAdapter>().on_update_player_state(move |player_loc| {
        let new_main_window = main_window_weak.unwrap();
        let field_adapter = new_main_window.global::<FieldAdapter>();

        let (ver_state, hor_state) = utils::get_ver_hor_state(player_loc, field_adapter.get_number_of_tiles());

        field_adapter.set_player_on_ver(ver_state);
        field_adapter.set_player_on_hor(hor_state);
    });
}
