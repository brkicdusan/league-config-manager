#![windows_subsystem = "windows"]

mod app;
mod champion;
mod colors;
mod config;
mod dialog;
mod error;
mod game_settings;
mod message;
mod paste;
mod profile;
mod theme;
mod websocket;
mod widget;

use std::sync::Arc;

use crate::theme::Theme;

use app::App;
use iced::{advanced::graphics::image::image_rs::ImageFormat, window, Size};
use tokio::sync::Mutex;

fn main() -> Result<(), iced::Error> {
    iced::application("League Config Manager", App::update, App::view)
        .theme(|_| Theme)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .window(window::Settings {
            size: Size {
                width: 600f32,
                height: 800f32,
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
