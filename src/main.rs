mod config;
pub mod utils;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    let new_field = config::read_config("./test_field.json").unwrap();

    main_window.set_field_top(new_field.field_top());
    main_window.set_field_right(new_field.field_right());
    main_window.set_field_left(new_field.field_left());
    main_window.set_field_bottom(new_field.field_bottom());

    main_window.run()
}
