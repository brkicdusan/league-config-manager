use iced::{
    widget::{button, container, text},
    Color, Element, Font, Length,
};

use crate::{message::Message, theme::Theme};

pub(crate) fn icon<'a>(codepoint: char) -> Element<'a, Message, Theme> {
    const ICON_FONT: Font = Font::with_name("icons");

    text(codepoint)
        .font(ICON_FONT)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .into()
}

pub(crate) fn open_icon<'a>() -> Element<'a, Message, Theme> {
    icon('\u{0e800}')
}

pub(crate) fn cancel_icon<'a>() -> Element<'a, Message, Theme> {
    icon('\u{0e802}')
}

pub(crate) fn confirm_icon<'a>() -> Element<'a, Message, Theme> {
    icon('\u{0e803}')
}

pub(crate) fn share_icon<'a>() -> Element<'a, Message, Theme> {
    icon('\u{0e801}')
}

pub(crate) fn add_icon<'a>() -> Element<'a, Message, Theme> {
    icon('\u{0e805}')
}

pub(crate) fn trash_icon<'a>() -> Element<'a, Message, Theme> {
    icon('\u{0e807}')
}

pub(crate) fn edit_icon<'a>() -> Element<'a, Message, Theme> {
    icon('\u{0e80C}')
}

pub(crate) fn use_icon<'a>() -> Element<'a, Message, Theme> {
    icon('\u{0e80B}')
}

pub(crate) fn import_icon<'a>() -> Element<'a, Message, Theme> {
    icon('\u{0e806}')
}

pub(crate) const SIZE_LEN: Length = Length::Fixed(45f32);

pub(crate) fn icon_btn(
    ico: Element<'_, Message, Theme>,
    msg: Option<Message>,
    col: Color,
) -> button::Button<'_, Message, Theme> {
    button(container(ico).center_y().center_x())
        .height(SIZE_LEN)
        .width(SIZE_LEN)
        .on_press_maybe(msg)
        .style(crate::theme::Button::Fill(col))
}
