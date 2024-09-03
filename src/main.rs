//![windows_subsystem = "windows"]

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
    success: bool,
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
                success: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("League Config Manager")
    }
    fn update(&mut self, message: Self::Message) -> Command<Message> {
        self.success = false;
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
            Message::RemoveProfile(s) => {
                for (i, p) in self.profiles.iter().enumerate() {
                    if p.get_name() == &s {
                        p.delete();
                        self.profiles.remove(i);
                        break;
                    }
                }
                Command::none()
            }
            Message::UseProfile(prof) => {
                if let Some(cfg) = &self.cfg {
                    prof.move_files(cfg);
                    cfg.set_readonly(self.readonly);
                    self.success = true;
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

        let mut profiles = column![].align_items(iced::Alignment::Center).spacing(15);

        for p in &self.profiles {
            profiles = profiles.push(p.get_item(&self.cfg));
        }

        let mut add_profile = button(text("Add profile"));
        if self.cfg.is_some() {
            add_profile = add_profile.on_press(Message::AddProfile);
        }

        let mut content =
            column![location, cb, Rule::horizontal(0), profiles, add_profile].spacing(10);

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

        if self.success {
            let success_text = text("Success!").style(theme::Text::Color(color!(0, 255, 0)));
            let success_container = container(success_text).center_x().width(Length::Fill);

            content = content.push(success_container);
        }

        container(content).padding(10).center_x().center_y().into()
    }
}
