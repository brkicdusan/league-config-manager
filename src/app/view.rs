use crate::*;

use champion::get_champion_name_from_id;
use iced::widget::container;
use widget::SIZE_LEN;

use iced::Length;

use error::Error;

use iced::widget::text;

use widget::import_icon;

use widget::add_icon;

use iced::widget::Checkbox;

use widget::open_icon;

use widget::icon_btn;

use iced::widget::tooltip;

use iced::widget::text_input;

use iced::widget::{column, row, Rule};

use crate::theme::Theme;

use message::Message;

use super::App;

impl App {
    pub(crate) fn view(&self) -> iced::Element<'_, Message, Theme> {
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
                Error::ChampionTaken => "Another profile already handles that champion",
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
}
