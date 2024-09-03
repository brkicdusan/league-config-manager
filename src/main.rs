#![windows_subsystem = "windows"]

mod cfg;
mod config;
mod dialog;
mod error;
mod message;
mod profile;

use cfg::Cfg;
use config::Config;
use error::Error;
use message::Message;
use profile::Profile;

use std::path::Path;

use iced::{
    color, executor, theme,
    widget::{button, column, container, row, text, text_input, Checkbox, Rule},
    Application, Command, Length, Settings, Theme,
};

fn main() -> Result<(), iced::Error> {
    Window::run(Settings::default())
}

struct Window {
    config: Config,
    cfg: Option<Cfg>,
    readonly: bool,
    error: Option<Error>,
    profiles: Vec<Profile>,
}

impl Window {
    fn set_cfg(&mut self, location: &Path) {
        match Cfg::new(location) {
            Ok(cfg) => {
                self.readonly = cfg.get_readonly();
                self.cfg = Some(cfg);
                self.error = None;
            }
            Err(e) => {
                self.cfg = None;
                self.error = Some(e);
            }
        }
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
        let mut err = None;
        let profiles = Profile::get_profiles();
        match Cfg::from_config(&conf) {
            Ok(c) => {
                readonly = c.get_readonly();
                cfg = Some(c);
            }
            Err(e) => err = Some(e),
        }
        (
            Window {
                config: conf,
                cfg,
                readonly,
                error: err,
                profiles,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("League Config Manager")
    }
    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::FindLocation => {
                Command::perform(dialog::find_config_dialog(), Message::SetLocation)
            }
            Message::SetLocation(Ok(location)) => {
                self.set_cfg(&location);
                self.config.set_path(Some(location));
                Command::none()
            }
            Message::SetLocation(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
            Message::SetReadonly(readonly) => {
                self.readonly = readonly;
                if let Some(c) = &self.cfg {
                    c.set_readonly(readonly);
                }
                Command::none()
            }
            Message::AddProfile => {
                if let Some(cfg) = &self.cfg {
                    let new_profile = Profile::new(cfg);
                    self.profiles.push(new_profile);
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

        let mut profile_row = row![].align_items(iced::Alignment::Center).spacing(15);

        for p in &self.profiles {
            profile_row = profile_row.push(text(p.get_name()));
        }

        let mut add_profile = button(text("Add profile"));
        if self.cfg.is_some() {
            add_profile = add_profile.on_press(Message::AddProfile);
        }

        let mut content =
            column![location, cb, Rule::horizontal(0), profile_row, add_profile].spacing(10);

        if let Some(e) = &self.error {
            let error_str = match e {
                Error::DialogClosed => "Dialog closed without choosing the folder",
                Error::WrongPath => "Wrong path",
                Error::MissingPath => "Missing path",
            };

            let error_text = text(error_str).style(theme::Text::Color(color!(200, 0, 0)));
            let error_container = container(error_text).center_x().width(Length::Fill);

            content = content.push(error_container);
        }

        container(content).padding(10).center_x().center_y().into()
    }
}
