pub mod app;
pub mod data;
pub mod components;
pub mod constants;
pub mod utils;

use iced::{application, window};
use iced_phosphor::{FONT_REGULAR, FONT_FILL};
use dotenv::dotenv;

use app::App;

fn main() -> iced::Result {
    dotenv().ok();

    application(App::new, App::update, App::view)
        .window(
            window::Settings {
                size: iced::Size::new(1200.0, 800.0),
                minimizable: true,
                min_size: Some(iced::Size::new(700.0, 800.0)),
                ..Default::default()
            }
        )
        .title(App::title)
        .theme(App::theme)
        .font(FONT_REGULAR)
        .font(FONT_FILL)
        .centered()
        .run()
}
