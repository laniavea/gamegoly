mod utils;
mod config_field;
mod config_player;
mod slint_setter;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    let field_data = config_field::read_config("./test_field.json").unwrap();
    let player_data = config_player::read_config("./player.json").unwrap();

    let main_window_weak = main_window.as_weak();

    slint_setter::set_field(main_window_weak.clone(), &field_data);
    slint_setter::set_player(main_window_weak.clone(), &player_data);
    slint_setter::set_info_panel(main_window_weak.clone());

    main_window.global::<LowerPanelAdapter>().on_update_player_state(move |player_loc| {
        let main_window = main_window_weak.unwrap();

        let (ver_state, hor_state) = utils::get_ver_hor_state(player_loc, field_data.field_number_of_elems());

        main_window.global::<FieldAdapter>().set_player_on_ver(ver_state);
        main_window.global::<FieldAdapter>().set_player_on_hor(hor_state);
    });

    main_window.run()
}
