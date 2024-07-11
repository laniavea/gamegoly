mod config;
pub mod utils;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    let gamegoly_data = config::read_config("./test_field.json").unwrap();

    main_window.set_field_top(gamegoly_data.field_top());
    main_window.set_field_right(gamegoly_data.field_right());
    main_window.set_field_left(gamegoly_data.field_left());
    main_window.set_field_bottom(gamegoly_data.field_bottom());

    main_window.set_main_info_title(gamegoly_data.main_info_title());

    main_window.run()
}
