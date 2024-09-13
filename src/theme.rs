use crate::colors;
use iced::widget::{button, checkbox, container, rule, text, text_input};
use iced::{application, Background, Border, Color};

#[derive(Debug, Clone, Copy, Default)]
pub struct Theme;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: colors::BG,
            text_color: colors::TEXT,
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: colors::TEXT.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Primary,
    Fill(Color),
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match style {
            Button::Primary => button::Appearance {
                border: Border {
                    color: colors::BLUE,
                    width: 5.0,
                    radius: 5.into(),
                },
                ..Default::default()
            },
            Button::Fill(col) => button::Appearance {
                border: Border {
                    color: *col,
                    width: 5.0,
                    radius: 5.into(),
                },
                background: Background::Color(*col).into(),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TextInput {
    #[default]
    Default,
}

impl text_input::StyleSheet for Theme {
    // TODO: style this
    type Style = TextInput;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Default => text_input::Appearance {
                background: colors::BG.into(),
                border: Border {
                    color: colors::BLUE,
                    width: 5.0,
                    radius: 5.into(),
                },
                icon_color: colors::TEXT,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Default => text_input::Appearance {
                background: colors::BG.into(),
                border: Border {
                    color: colors::GOLD,
                    width: 2.0,
                    radius: 5.into(),
                },
                icon_color: colors::GOLD_LIGHT,
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Default => text_input::Appearance {
                background: colors::BG.into(),
                border: Border {
                    color: colors::BLUE,
                    width: 5.0,
                    radius: 5.into(),
                },
                icon_color: colors::TEXT.into(),
            },
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        colors::GOLD_LIGHT
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        colors::TEXT
    }

    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        colors::GOLD_LIGHT
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        colors::TEXT
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: colors::BG.into(),
            border: Border {
                color: colors::BLUE,
                width: 1.0,
                radius: 5.0.into(),
            },
            icon_color: colors::BLUE,
            text_color: colors::BLUE.into(),
        }
    }

    fn disabled(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: colors::BG.into(),
            border: Border {
                color: colors::BLUE,
                width: 1.0,
                radius: 5.0.into(),
            },
            icon_color: colors::BLUE,
            text_color: colors::BLUE.into(),
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: colors::BG.into(),
            border: Border {
                color: colors::BLUE,
                width: 1.0,
                radius: 5.0.into(),
            },
            icon_color: colors::BLUE,
            text_color: colors::BLUE.into(),
        }
    }
}

impl rule::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: colors::BLUE,
            width: 1,
            radius: 1.into(),
            fill_mode: rule::FillMode::Full,
        }
    }
}
