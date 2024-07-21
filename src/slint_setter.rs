use crate::{AppWindow, FieldAdapter, InfoPanelAdapter, utils};
use crate::config_field::FieldDataSlint;
use crate::config_player::PlayerData;

use slint::Weak;
use slint::ComponentHandle;

pub fn set_field(window: Weak<AppWindow>, field_data: &FieldDataSlint) {
    let main_window = window.unwrap();
    let field_adapter = main_window.global::<FieldAdapter>();

    field_adapter.set_field_top(field_data.field_top());
    field_adapter.set_field_right(field_data.field_right());
    field_adapter.set_field_left(field_data.field_left());
    field_adapter.set_field_bottom(field_data.field_bottom());
    field_adapter.set_number_of_tiles(field_data.field_number_of_elems());
    field_adapter.set_main_info_title(field_data.main_info_title());
}

pub fn set_player(window: Weak<AppWindow>, player_data: &PlayerData) {
    let main_window = window.unwrap();
    let field_adapter = main_window.global::<FieldAdapter>();

    field_adapter.set_player_loc_id(player_data.location());

    let (ver_state, hor_state) = utils::get_ver_hor_state(player_data.location(), field_adapter.get_number_of_tiles());

    field_adapter.set_player_on_ver(ver_state);
    field_adapter.set_player_on_hor(hor_state);

} 

pub fn set_info_panel(window: Weak<AppWindow>) {
    let main_window = window.unwrap();
    let info_panel_adapter = main_window.global::<InfoPanelAdapter>();
    info_panel_adapter.set_panel_mode(2);
}
