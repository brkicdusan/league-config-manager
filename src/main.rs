#![windows_subsystem = "windows"]

mod cfg;
mod config;
mod error;

use cfg::Cfg;
use config::Config;

use std::path::{Path, PathBuf};

use iced::{
    executor,
    widget::{button, column, container, row, text, text_input, Checkbox},
    Application, Command, Settings, Theme,
};

fn main() -> Result<(), iced::Error> {
    Window::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    FindLocation,
    SetLocation(Result<PathBuf, error::Error>),
    SetReadonly(bool),
}

struct Window {
    config: Config,
    cfg: Option<Cfg>,
    readonly: bool,
}

impl Window {
    fn set_cfg(&mut self, location: &Path) {
        if let Ok(cfg) = Cfg::new(location) {
            self.cfg = Some(cfg)
        } else {
            self.cfg = None
        }

        // TODO:error handling missing
    }
}

impl Application for Window {
    type Message = Message;
    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Window, iced::Command<Message>) {
        let conf = Config::new();
        let mut cfg = None;
        let mut readonly = false;
        if let Ok(c) = Cfg::from_config(&conf) {
            readonly = c.get_readonly();
            cfg = Some(c);
        }
        (
            Window {
                config: conf,
                cfg,
                readonly,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("League Config Manager")
    }
    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::FindLocation => Command::perform(find_config_dialog(), Message::SetLocation),
            Message::SetLocation(Ok(location)) => {
                self.set_cfg(&location);
                self.config.set_path(Some(location));
                Command::none()
            }
            Message::SetLocation(Err(error)) => {
                println!("{:?}", error);
                // TODO: error handling
                Command::none()
            }
            Message::SetReadonly(readonly) => {
                self.readonly = readonly;
                if let Some(c) = &self.cfg {
                    c.set_readonly(readonly);
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let config_path = text_input("Config not found", self.config.path_to_str()).padding(10);
        let location_btn = button(text("Find \"League of Legends\" folder"))
            .padding(10)
            .on_press(Message::FindLocation);
        let location = row![config_path, location_btn]
            .align_items(iced::Alignment::Center)
            .spacing(10);

        let mut cb = Checkbox::new("Lock settings", self.readonly);
        if self.cfg.is_some() {
            cb = cb.on_toggle(Message::SetReadonly)
        }

        let tmp_text = if let Some(c) = &self.cfg {
            text(c.game.to_str().expect("Should always convert"))
        } else {
            text("Config has not been located")
        };

        container(column![location, cb, tmp_text])
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}

/// Opens dialog to locate config.
///
/// # Errors
/// When dialog closes return Error::DialogClosed
async fn find_config_dialog() -> Result<std::path::PathBuf, error::Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Find \"League of Legends\" folder")
        .pick_folder()
        .await
        .ok_or(error::Error::DialogClosed)?;
    Ok(handle.path().to_owned())
}

// G:\Riot Games\League of Legends
