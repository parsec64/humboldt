mod app;

use dotenv::dotenv;
use iced::{Font, application, window};

use crate::app::App;


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
        // .font(icon::FONT)
        .default_font(Font::MONOSPACE)
        .centered()
        .run()
}
