use crate::*;

use iced::Alignment::Center;
use widget::{cancel_icon, confirm_icon};

use iced::widget::text_input;

use crate::widget::edit_icon;

use iced::widget::{column, row};

use iced::widget::container;

use crate::widget::share_icon;

use crate::widget::use_icon;

use crate::widget::trash_icon;

use crate::widget::icon_btn;

use iced::widget::tooltip;

use crate::game_settings::GameSettings;

use iced::widget::pick_list;

use iced::Length;

use iced::widget::text;

use crate::theme::Theme;

use crate::message::Message;

use iced::Element;

use super::Profile;

//view
impl Profile {
    pub(crate) fn champion_row(&self) -> Element<Message, Theme> {
        let txt = text("Swap when this champion is selected:").width(Length::Fill);
        let options = Self::options_list();
        let pl = pick_list(options, Some(self.selected()), |s| {
            Message::PickListChange(self.name.clone(), s)
        });
        row![txt, pl].align_y(Center).into()
    }

    pub fn get_item(&self, cfg: &Option<GameSettings>) -> Element<Message, Theme> {
        let del_btn = tooltip(
            icon_btn(
                trash_icon(),
                Message::RemoveProfile(self.name.clone()).into(),
                colors::RED,
            ),
            "Delete profile",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let use_msg = if cfg.is_some() {
            Some(Message::UseProfile(self.clone()))
        } else {
            None
        };

        let use_btn = tooltip(
            icon_btn(use_icon(), use_msg, colors::BLUE),
            "Use this profile",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let export_btn = tooltip(
            icon_btn(
                share_icon(),
                Message::Export(self.clone()).into(),
                colors::BLUE,
            ),
            "Export settings to .zip",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let edit_btn = if !self.editing {
            container(
                tooltip(
                    icon_btn(
                        edit_icon(),
                        Message::Edit(self.name.clone()).into(),
                        colors::BLUE,
                    ),
                    "Edit name",
                    tooltip::Position::Bottom,
                )
                .class(theme::Container::Tooltip),
            )
        } else {
            container(
                row![
                    icon_btn(
                        confirm_icon(),
                        Message::Confirm(self.name.clone()).into(),
                        colors::GREEN
                    ),
                    icon_btn(
                        cancel_icon(),
                        Message::Reset(self.name.clone()).into(),
                        colors::RED
                    )
                ]
                .spacing(10),
            )
        };

        let mut profile_row = row![].spacing(10).align_y(iced::Alignment::Center);

        if !self.editing {
            profile_row = profile_row.push(text(&self.name).width(iced::Length::Fill));
        } else {
            profile_row = profile_row.push(
                text_input("", &self.edit_name)
                    .padding(10)
                    .on_input(|s| Message::OnChange(self.name.clone(), s))
                    .width(iced::Length::Fill),
            );
        }

        profile_row = profile_row.push(edit_btn);
        profile_row = profile_row.push(use_btn);
        profile_row = profile_row.push(export_btn);
        profile_row = profile_row.push(del_btn);

        column![profile_row, self.champion_row()].spacing(10).into()
    }
}
