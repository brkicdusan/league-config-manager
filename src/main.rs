#![windows_subsystem = "windows"]

mod app;
mod cfg;
mod champion;
mod colors;
mod config;
mod dialog;
mod error;
mod message;
mod profile;
mod theme;
mod websocket;
mod widget;

use crate::theme::Theme;

use app::App;
use iced::{advanced::graphics::image::image_rs::ImageFormat, window, Size};

fn main() -> Result<(), iced::Error> {
    iced::application("League Config Manager", App::update, App::view)
        .theme(|_| Theme)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .window(window::Settings {
            size: Size {
                width: 500f32,
                height: 600f32,
            },
            icon: window::icon::from_file_data(
                include_bytes!("../assets/mobile-logo.png"),
                Some(ImageFormat::Png),
            )
            .map(Some)
            .unwrap_or(None),
            ..window::Settings::default()
        })
        .subscription(App::subscription)
        .run_with(App::new)
}
