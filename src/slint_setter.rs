use crate::{AppWindow, FieldAdapter, InfoPanelAdapter, LowerPanelAdapter, utils};
use crate::config_field::{FieldTilesDataSlint, FieldMainDataSlint};
use crate::config_player::PlayerDataSlint;

use slint::Weak;
use slint::ComponentHandle;

pub fn set_field_tiles(window: Weak<AppWindow>, field_tiles: &FieldTilesDataSlint) {
    let main_window = window.unwrap();
    let field_adapter: FieldAdapter = main_window.global::<FieldAdapter>();

    field_adapter.set_field_top(field_tiles.field_top());
    field_adapter.set_field_right(field_tiles.field_right());
    field_adapter.set_field_left(field_tiles.field_left());
    field_adapter.set_field_bottom(field_tiles.field_bottom());
    field_adapter.set_number_of_tiles(field_tiles.field_number_of_elems());
}

pub fn set_field_main_info(window: Weak<AppWindow>, field_main_data: &FieldMainDataSlint) {
    let main_window = window.unwrap();
    let field_adapter: FieldAdapter = main_window.global::<FieldAdapter>();

    field_adapter.set_main_info_title(field_main_data.main_title());
    field_adapter.set_base_dice(field_main_data.base_dice());
    field_adapter.set_help_data(field_main_data.help_data());
    field_adapter.set_conditions(field_main_data.conditions());
    field_adapter.set_lists(field_main_data.static_lists());
    field_adapter.set_special_dices(field_main_data.special_dices());
}

pub fn set_player(window: Weak<AppWindow>, player_data: &PlayerDataSlint) {
    let main_window = window.unwrap();
    let field_adapter = main_window.global::<FieldAdapter>();
    let lower_panel_adapter = main_window.global::<LowerPanelAdapter>();

    field_adapter.set_player_loc_id(player_data.location());
    field_adapter.set_player_drops(player_data.drops());
    field_adapter.set_player_half_moves(player_data.half_moves());
    lower_panel_adapter.set_player_status(player_data.state());

    lower_panel_adapter.set_player_special(player_data.specials());
    lower_panel_adapter.set_player_add_tags(player_data.add_tags());

    let (ver_state, hor_state) = utils::get_ver_hor_state(player_data.location(), field_adapter.get_number_of_tiles());

    field_adapter.set_player_on_ver(ver_state);
    field_adapter.set_player_on_hor(hor_state);

    lower_panel_adapter.set_combined_specials(utils::combine_strings(lower_panel_adapter.get_player_special()));
    lower_panel_adapter.set_combined_add_tags(utils::combine_strings(lower_panel_adapter.get_player_add_tags()));
} 

pub fn set_info_panel(window: Weak<AppWindow>) {
    let main_window = window.unwrap();
    let info_panel_adapter = main_window.global::<InfoPanelAdapter>();
    info_panel_adapter.set_panel_mode(5);
}

pub fn set_lower_panel(window: Weak<AppWindow>) {
    let main_window = window.unwrap();
    let lower_panel_adapter = main_window.global::<LowerPanelAdapter>();
    lower_panel_adapter.set_condition_button(false);
}
