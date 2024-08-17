mod utils;
mod config_field;
mod config_player;
mod slint_setter;
mod slint_callbacks;
mod slint_sturcts;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    let (field_tiles, field_main_data) = 
        match config_field::read_config("./test_field.json") {
            Err(err) => {
                eprintln!("Exited with next error while field config read: {err}");
                return Ok(())
            },
            Ok((tiles, main_data)) => (tiles, main_data),
        };

    let player_data = match config_player::read_config("./player.json") {
        Err(err) => {
            eprintln!("Exited with next error while player config read: {err}");
            return Ok(())
        },
        Ok(player_data) => player_data,
    };

    let main_window_weak = main_window.as_weak();

    slint_setter::set_field_tiles(main_window_weak.clone(), &field_tiles);
    slint_setter::set_field_main_info(main_window_weak.clone(), &field_main_data);
    slint_setter::set_player(main_window_weak.clone(), &player_data);
    slint_setter::set_info_panel(main_window_weak.clone());

    slint_callbacks::lower_panel_callbacks(main_window_weak.clone());
    slint_callbacks::field_callbacks(main_window_weak.clone());

    main_window.run()
}
