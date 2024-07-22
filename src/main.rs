mod utils;
mod config_field;
mod config_player;
mod slint_setter;
mod slint_callbacks;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    let field_data = config_field::read_config("./test_field.json").unwrap();
    let player_data = config_player::read_config("./player.json").unwrap();

    let main_window_weak = main_window.as_weak();

    slint_setter::set_field(main_window_weak.clone(), &field_data);
    slint_setter::set_player(main_window_weak.clone(), &player_data);
    slint_setter::set_info_panel(main_window_weak.clone());

    slint_callbacks::lower_panel_callbacks(main_window_weak.clone());

    main_window.run()
}
