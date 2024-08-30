mod utils;
mod config_field;
mod config_player;
mod slint_setter;
mod slint_callbacks;
mod slint_structs;

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
    slint_setter::set_lower_panel(main_window_weak.clone());

    slint_callbacks::lower_panel_callbacks(main_window_weak.clone());
    slint_callbacks::field_callbacks(main_window_weak.clone());
    slint_callbacks::info_panel_callbacks(main_window_weak.clone());

    main_window.run()
}

// Notes for slint:
// 1) Try to separate structs and adapters to non-related files to not meet import loop, slint can't
//    handle it itself
//
// 2) I only found solution to set size of imported objects based on parent object is to create
//    property by hands and pass to it sizes
//
// 3) Maybe downcasts aren't necessary :(
//
// 4) When itering for the vec inside slint file you can use 
// for smth[i] in anything_iterable {
//      ...
//          adapter.array_of_smth[i] = false // But element i MUST exist before calling it
//      ...
// }
// to access number i and bind smth to it (ex. info_panel.slint)
//
// 5) Enjoy
//
// for slint 1.6-1.7
