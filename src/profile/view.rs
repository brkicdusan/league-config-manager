use crate::*;

use iced::Alignment::Center;
use widget::{cancel_icon, confirm_icon};

use iced::widget::{horizontal_space, text_input, Row};

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

use iced::{padding, Length};

use iced::widget::text;

use crate::theme::Theme;

use crate::message::Message;

use iced::Element;

use super::Profile;

//view
impl Profile {
    fn champion_row(&self) -> Element<Message, Theme> {
        let txt = text("Swap when this champion is selected:").width(Length::Fill);
        let options = Self::options_list();
        let pl = pick_list(options, Some(self.selected()), |s| {
            Message::PickListChange(self.name.clone(), s)
        });
        row![txt, pl]
            .padding(padding::top(5))
            .align_y(Center)
            .into()
    }

    fn share_row(&self) -> Row<Message, Theme> {
        let mut txt = self.last_link.clone();
        if self.last_link.is_empty() {
            txt = "No link generated".to_string();
        }
        let txt = tooltip(
            text(txt),
            "Last generated profile link",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let copy_btn = tooltip(
            icon_btn(
                text("C").into(),
                Some(Message::CopyLink(self.last_link.clone())),
                colors::BLUE,
            ),
            "Copy link",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let gen_btn = tooltip(
            icon_btn(
                text("G").into(),
                Some(Message::GenerateLink(
                    self.game_settings.to_paste_string(),
                    self.name().clone(),
                )),
                colors::BLUE,
            ),
            "Generate link",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        row![txt, horizontal_space(), copy_btn, gen_btn]
            .align_y(Center)
            .spacing(10)
    }

    pub fn view_profile(&self, cfg: &Option<GameSettings>) -> Element<Message, Theme> {
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

        column![profile_row, self.champion_row(), self.share_row()]
            .spacing(10)
            .into()
    }
}
