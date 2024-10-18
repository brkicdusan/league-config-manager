//#![windows_subsystem = "windows"]

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

use cfg::Cfg;
use champion::get_champion_name_from_id;
use config::Config;
use dialog::export_zip_path;
use error::Error;
use message::Message;
use profile::Profile;
use websocket::connect;
use widget::{add_icon, icon_btn, import_icon, open_icon, SIZE_LEN};

use std::path::Path;

use crate::theme::Theme;

use iced::{
    advanced::graphics::image::image_rs::ImageFormat,
    widget::{column, container, row, text, text_input, tooltip, Checkbox, Rule},
    window::{self},
    Length, Size, Subscription, Task,
};

#[tokio::main]
async fn main() -> Result<(), iced::Error> {
    iced::application("League Config Manager", Window::update, Window::view)
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
        .subscription(Window::subscription)
        .run_with(Window::new)
}

struct Window {
    config: Config,
    cfg: Option<Cfg>,
    readonly: bool,
    error: Option<Error>,
    profiles: Vec<Profile>,
    success: Option<String>,
    champion_id: Option<u32>,
    connected: bool,
    retry_in: Option<u32>,
}

impl Window {
    fn set_cfg(&mut self, location: &Path) -> Option<Error> {
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
        self.error
    }

    fn get_profile_from_name(&mut self, name: &String) -> Option<&mut Profile> {
        self.profiles.iter_mut().find(|p| p.get_name().eq(name))
    }
}

impl Window {
    fn new() -> (Window, iced::Task<Message>) {
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
                success: None,
                champion_id: None,
                connected: false,
                retry_in: None,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        self.success = None;
        // TODO: fix errors disappearing
        self.error = None;

        match message {
            Message::FindLocation => {
                Task::perform(dialog::find_config_dialog(), Message::SetLocation)
            }
            Message::SetLocation(Ok(location)) => {
                if self.set_cfg(&location).is_none() {
                    self.config.set_path(Some(location));
                }
                Task::none()
            }
            Message::SetLocation(Err(error)) => {
                self.error = Some(error);
                Task::none()
            }
            Message::SetReadonly(readonly) => {
                self.readonly = readonly;
                if let Some(c) = &self.cfg {
                    c.set_readonly(readonly);
                }
                Task::none()
            }
            Message::AddProfile => {
                if let Some(cfg) = &self.cfg {
                    let new_profile = Profile::new(cfg);
                    self.profiles.push(new_profile);
                }
                Task::none()
            }
            Message::RemoveProfile(s) => {
                for (i, p) in self.profiles.iter().enumerate() {
                    if p.get_name() == &s {
                        p.delete();
                        self.profiles.remove(i);
                        break;
                    }
                }
                Task::none()
            }
            Message::UseProfile(prof) => {
                if let Some(cfg) = &self.cfg {
                    prof.copy_files(cfg);
                    cfg.set_readonly(self.readonly);
                    self.success = Some(format!("Using \"{}\"", prof.get_name()))
                }
                Task::none()
            }
            Message::Edit(name) => {
                let prof = self.get_profile_from_name(&name).unwrap();
                prof.edit_start();
                Task::none()
            }
            Message::Confirm(name) => {
                let prof = self.get_profile_from_name(&name).unwrap();

                if let Err(e) = prof.edit_confirm() {
                    self.error = Some(e);
                } else {
                    self.success = Some(format!("Changed name to {}", &prof.get_name()));
                    self.error = None;
                }
                Task::none()
            }
            Message::Reset(name) => {
                let prof = self.get_profile_from_name(&name).unwrap();
                prof.edit_reset();
                Task::none()
            }
            Message::OnChange(name, new_name) => {
                let prof = self.get_profile_from_name(&name).unwrap();
                prof.edit_change(new_name);
                Task::none()
            }
            Message::Export(profile) => Task::perform(export_zip_path(profile), Message::SetExport),
            Message::SetExport(Ok((export_path, profile))) => {
                match profile.zip(export_path) {
                    Ok(_) => self.success = Some("Exported profile".to_string()),
                    Err(_) => self.error = Some(Error::ZipExport),
                };
                Task::none()
            }
            Message::SetExport(Err(e)) => {
                self.error = Some(e);
                Task::none()
            }
            Message::SetImport(Ok(import_path)) => {
                match Profile::from_zip(&import_path) {
                    Ok(profile) => self.profiles.push(profile),
                    Err(_) => self.error = Some(Error::ZipImport),
                }
                Task::none()
            }
            Message::SetImport(Err(e)) => {
                self.error = Some(e);
                Task::none()
            }
            Message::Import => Task::perform(dialog::import_zip_path(), Message::SetImport),
            Message::WebsocketEvent(event) => {
                match event {
                    websocket::Event::Selected(x) => {
                        self.champion_id = None;

                        if x > 0 {
                            self.champion_id = Some(x);
                        }
                    }
                    websocket::Event::Connected => {
                        self.connected = true;
                        self.retry_in = None;
                    }
                    websocket::Event::Disconnected => {
                        self.connected = false;
                        self.retry_in = None;
                    }
                    websocket::Event::Retrying(t) => self.retry_in = Some(t),
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message, Theme> {
        let config_path = text_input("Config not found", self.config.path_to_str()).padding(10);
        let location_btn = tooltip(
            icon_btn(open_icon(), Message::FindLocation.into(), colors::GOLD),
            "Find \"League of Legends\" directory",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let mut cb = Checkbox::new("Lock settings", self.readonly);
        if self.cfg.is_some() {
            cb = cb.on_toggle(Message::SetReadonly)
        }

        let cb = tooltip(
            cb,
            "Settings can't be changed in game while this is active",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let mut profiles = column![].align_x(iced::Alignment::Center).spacing(15);

        for p in &self.profiles {
            profiles = profiles.push(p.get_item(&self.cfg));
        }

        let mut add_profile = icon_btn(add_icon(), None, colors::GOLD);
        if self.cfg.is_some() {
            add_profile = add_profile.on_press(Message::AddProfile);
        }

        let add_profile = tooltip(
            add_profile,
            "Add current settings",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let import_profile = tooltip(
            icon_btn(import_icon(), Message::Import.into(), colors::BLUE),
            "Import profile from .zip file",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let location = row![config_path, location_btn, add_profile, import_profile]
            .align_y(iced::Alignment::Center)
            .spacing(10);

        let champion_text = text(if self.connected {
            let mut txt = "Connected.".to_string();
            if self.champion_id.is_some() {
                txt = format!(
                    "Selected champion: {}",
                    get_champion_name_from_id(self.champion_id.unwrap()).unwrap()
                );
            }
            txt
        } else {
            let mut txt = "Disconnected.".to_string();
            if self.retry_in.is_some() {
                txt = format!(
                    "{} {} {} seconds...",
                    txt,
                    "Retrying in",
                    self.retry_in.unwrap()
                );
            }
            txt
        });

        let mut content =
            column![location, cb, Rule::horizontal(0), profiles, champion_text].spacing(10);

        if let Some(e) = &self.error {
            let error_str = match e {
                Error::DialogClosed => "Dialog closed without choosing the folder",
                Error::WrongPath => {
                    "Wrong path (folder should look like \".../Riot Games/League of Legends/\""
                }
                Error::MissingPath => {
                    "Missing path (choose the League of Legends install directory)"
                }
                Error::NameTaken => "Name is taken",
                Error::ZipExport => "Error exporting profile",
                Error::ZipImport => "Error importing profile",
            };

            let error_text = text(error_str).size(20).class(theme::Text::Error);
            let error_container = container(error_text)
                .class(crate::theme::Container::Error)
                .center_x(Length::Fill)
                .center_y(SIZE_LEN);

            content = content.push(error_container);
        }

        if let Some(success) = &self.success {
            let success_text = text(format!("Success! - {}", success))
                .size(20)
                .class(theme::Text::Success);
            let success_container = container(success_text)
                .class(theme::Container::Success)
                .center_x(Length::Fill)
                .center_y(SIZE_LEN);

            content = content.push(success_container);
        }

        container(content).padding(10).center_x(Length::Fill).into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(connect).map(Message::WebsocketEvent)
    }
}
