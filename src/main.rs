//![windows_subsystem = "windows"]

mod cfg;
mod colors;
mod config;
mod dialog;
mod error;
mod message;
mod profile;
mod theme;
mod widget;

use cfg::Cfg;
use config::Config;
use dialog::export_zip_path;
use error::Error;
use message::Message;
use profile::Profile;
use widget::{add_icon, icon_btn, import_icon, open_icon, SIZE_LEN};

use std::path::Path;

use crate::theme::Theme;

use iced::{
    executor,
    widget::{button, column, container, row, text, text_input, Checkbox, Rule},
    Application, Command, Font, Length, Settings,
};

fn main() -> Result<(), iced::Error> {
    Window::run(Settings {
        fonts: vec![include_bytes!("../fonts/icons.ttf").as_slice().into()],
        // default_font: Font::MONOSPACE,
        ..Settings::default()
    })
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

    fn get_profile_from_name(&mut self, name: &String) -> Option<&mut Profile> {
        self.profiles.iter_mut().find(|p| p.get_name().eq(name))
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
                    prof.copy_files(cfg);
                    cfg.set_readonly(self.readonly);
                    self.success = true;
                }
                Command::none()
            }
            Message::Edit(name) => {
                let prof = self.get_profile_from_name(&name).unwrap();
                prof.edit_start();
                Command::none()
            }
            Message::Confirm(name) => {
                let prof = self.get_profile_from_name(&name).unwrap();
                if let Err(e) = prof.edit_confirm() {
                    self.error = Some(e);
                } else {
                    self.error = None;
                    self.success = true;
                }
                Command::none()
            }
            Message::Reset(name) => {
                let prof = self.get_profile_from_name(&name).unwrap();
                prof.edit_reset();
                Command::none()
            }
            Message::OnChange(name, new_name) => {
                let prof = self.get_profile_from_name(&name).unwrap();
                prof.edit_change(new_name);
                Command::none()
            }
            Message::Export(profile) => {
                Command::perform(export_zip_path(profile), Message::SetExport)
            }
            Message::SetExport(Ok((export_path, profile))) => {
                match profile.zip(export_path) {
                    Ok(_) => self.success = true,
                    Err(_) => self.error = Some(Error::ZipExport),
                };
                Command::none()
            }
            Message::SetExport(Err(e)) => {
                self.error = Some(e);
                Command::none()
            }
            Message::SetImport(Ok(import_path)) => {
                match Profile::from_zip(&import_path) {
                    Ok(profile) => self.profiles.push(profile),
                    Err(_) => self.error = Some(Error::ZipImport),
                }
                Command::none()
            }
            Message::SetImport(Err(e)) => {
                self.error = Some(e);
                Command::none()
            }
            Message::Import => Command::perform(dialog::import_zip_path(), Message::SetImport),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Theme> {
        let config_path = text_input("Config not found", self.config.path_to_str()).padding(10);
        let location_btn = icon_btn(open_icon(), Message::FindLocation.into(), colors::GOLD);

        let mut cb = Checkbox::new("Lock settings", self.readonly);
        if self.cfg.is_some() {
            cb = cb.on_toggle(Message::SetReadonly)
        }

        let mut profiles = column![].align_items(iced::Alignment::Center).spacing(15);

        for p in &self.profiles {
            // profiles = profiles.push(p.get_item(&self.cfg));
        }

        let mut add_profile = icon_btn(add_icon(), None, colors::GOLD);
        if self.cfg.is_some() {
            add_profile = add_profile.on_press(Message::AddProfile);
        }

        let import_profile = icon_btn(import_icon(), Message::Import.into(), colors::BLUE);

        let location = row![config_path, location_btn, import_profile, add_profile]
            .align_items(iced::Alignment::Center)
            .spacing(10);

        let mut content = column![location, cb, Rule::horizontal(0), profiles].spacing(10);

        if let Some(e) = &self.error {
            let error_str = match e {
                Error::DialogClosed => "Dialog closed without choosing the folder",
                Error::WrongPath => "Wrong path",
                Error::MissingPath => "Missing path",
                Error::NameTaken => "Name is taken",
                Error::ZipExport => "Error exporting profile",
                Error::ZipImport => "Error importing profile",
            };

            let error_text = text(error_str)
                // .style(iced::theme::Text::Color(color!(200, 0, 0)))
            ;
            let error_container = container(error_text).center_x().width(Length::Fill);

            content = content.push(error_container);
        }

        if self.success {
            let success_text = text("Success!")
                // .style(iced::theme::Text::Color(color!(0, 255, 0)))
            ;
            let success_container = container(success_text).center_x().width(Length::Fill);

            content = content.push(success_container);
        }

        container(content).padding(10).center_x().center_y().into()
        // container(location).padding(10).into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::default()
    }
}
