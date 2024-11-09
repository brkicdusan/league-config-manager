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
        let top_bar = self.view_top();

        let profiles = self.view_profiles();

        let connection_status = self.view_connection();

        let import_bar = self.view_import();

        let mut content = column![
            top_bar,
            Rule::horizontal(0),
            profiles,
            connection_status,
            Rule::horizontal(0),
            import_bar
        ]
        .spacing(10);

        content = content.push_maybe(self.view_error());

        content = content.push_maybe(self.view_success());

        container(content).padding(10).center_x(Length::Fill).into()
    }

    fn view_profiles(&self) -> iced::widget::Column<'_, Message, Theme> {
        let mut profiles = column![].align_x(iced::Alignment::Center).spacing(15);

        for p in &self.profiles {
            profiles = profiles.push(p.view_profile(&self.cfg));
            profiles = profiles.push(Rule::horizontal(0));
        }
        profiles
    }

    fn view_connection(&self) -> iced::widget::Text<'_, Theme> {
        let connection_status = text(if self.connected {
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
        connection_status
    }

    fn view_success(&self) -> Option<iced::widget::Container<'_, Message, Theme>> {
        if let Some(success) = &self.success {
            let success_text = text(format!("Success! - {}", success))
                .size(20)
                .class(theme::Text::Success);
            let success_container = container(success_text)
                .class(theme::Container::Success)
                .center_x(Length::Fill)
                .center_y(SIZE_LEN);
            return Some(success_container);
        }
        None
    }

    fn view_error(&self) -> Option<iced::widget::Container<'_, Message, Theme>> {
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
                Error::Import => "Error importing a profile",
            };

            let error_text = text(error_str).size(20).class(theme::Text::Error);
            let error_container = container(error_text)
                .class(crate::theme::Container::Error)
                .center_x(Length::Fill)
                .center_y(SIZE_LEN);

            return Some(error_container);
        }
        None
    }

    fn view_import(&self) -> iced::Element<'_, Message, Theme> {
        let text_inp = text_input("Import link", &self.link)
            .padding(10)
            .on_input(Message::ChangeLink)
            .on_paste(Message::ChangeLink);

        let import_button = tooltip(
            icon_btn(
                add_icon(),
                Some(Message::FetchLink(self.link.clone())),
                colors::BLUE,
            ),
            "Copy link",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        row![text_inp, import_button]
            .spacing(10)
            .align_y(iced::Alignment::Center)
            .into()
    }

    fn view_top(&self) -> iced::widget::Column<'_, Message, Theme> {
        let config_path = text_input("Config not found", self.config.path_to_str()).padding(10);
        let location_btn = tooltip(
            icon_btn(open_icon(), Message::FindLocation.into(), colors::GOLD),
            "Find \"League of Legends\" directory",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

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

        column![location, cb].spacing(10)
    }
}
