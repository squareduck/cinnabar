mod app;
mod config;
mod elements;
mod message;
mod state;

use app::App;
use iced::Font;

fn main() -> iced::Result {
    iced::application("Cinnabar", App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .default_font(Font::with_name("Iosevka Nerd Font"))
        .run_with(|| App::new())
}
