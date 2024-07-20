mod utils;
mod config_field;
mod config_player;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    let field_data = config_field::read_config("./test_field.json").unwrap();
    let player_data = config_player::read_config("./player.json").unwrap();

    main_window.global::<FieldAdapter>().set_field_top(field_data.field_top());
    main_window.global::<FieldAdapter>().set_field_right(field_data.field_right());
    main_window.global::<FieldAdapter>().set_field_left(field_data.field_left());
    main_window.global::<FieldAdapter>().set_field_bottom(field_data.field_bottom());
    main_window.global::<FieldAdapter>().set_number_of_tiles(field_data.field_number_of_elems());

    main_window.global::<FieldAdapter>().set_main_info_title(field_data.main_info_title());

    main_window.global::<FieldAdapter>().set_player_loc_id(player_data.location());

    let (ver_state, hor_state) = utils::get_ver_hor_state(player_data.location(), field_data.field_number_of_elems());

    main_window.global::<FieldAdapter>().set_player_on_ver(ver_state);
    main_window.global::<FieldAdapter>().set_player_on_hor(hor_state);

    main_window.global::<InfoPanelAdapter>().set_panel_mode(2);

    let main_window_weak = main_window.as_weak();

    main_window.on_update_player_state(move |player_loc| {
        let main_window = main_window_weak.unwrap();

        let (ver_state, hor_state) = utils::get_ver_hor_state(player_loc, field_data.field_number_of_elems());

        main_window.global::<FieldAdapter>().set_player_on_ver(ver_state);
        main_window.global::<FieldAdapter>().set_player_on_hor(hor_state);
    });

    main_window.run()
}
