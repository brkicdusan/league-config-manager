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

#[derive(Debug, Clone, Copy, Default)]
pub enum Text {
    #[default]
    Default,
    Error,
    Success,
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        let color = match style {
            // Text::Error => colors::RED,
            // Text::Success => colors::GREEN,
            _ => colors::TEXT,
        }
        .into();
        text::Appearance { color }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    Error,
    Success,
    Tooltip,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Error => container::Appearance {
                background: Background::Color(colors::RED).into(),
                border: Border {
                    color: colors::RED_BG,
                    width: 2.0,
                    radius: 5.into(),
                },
                text_color: colors::RED.into(),
                ..container::Appearance::default()
            },
            Container::Success => container::Appearance {
                background: Background::Color(colors::GREEN).into(),
                border: Border {
                    color: colors::GREEN_BG,
                    width: 2.0,
                    radius: 5.into(),
                },
                text_color: colors::GREEN.into(),
                ..container::Appearance::default()
            },
            Container::Tooltip => container::Appearance {
                background: Background::Color(colors::BG).into(),
                border: Border {
                    color: colors::BG,
                    width: 1.0,
                    radius: 10.into(),
                },

                text_color: colors::TEXT.into(),
                ..container::Appearance::default()
            },

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

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        let color = match style {
            Button::Fill(colors::GOLD) => colors::GOLD_LIGHT,
            Button::Fill(colors::GREEN) => colors::GREEN_BG,
            Button::Fill(colors::RED) => colors::RED_BG,
            _ => colors::BLUE_DARK,
        };

        button::Appearance {
            shadow_offset: active.shadow_offset + iced::Vector::new(0.0, 1.0),
            background: Background::Color(color)
            .into(),
            border: Border {
                color,
                width: 5.0,
                radius: 5.into(),
            },
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: iced::Vector::default(),
            ..active
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
                Background::Gradient(gradient) => Background::Gradient(gradient.mul_alpha(0.5)),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            border: Border {
                color: Color {
                    a: active.border.color.a * 0.5,
                    ..active.border.color
                },
                ..active.border
            },
            ..active
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TextInput {
    #[default]
    Default,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Default => text_input::Appearance {
                background: colors::BG.into(),
                border: Border {
                    color: colors::TEXT,
                    width: 1.0,
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
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        colors::GOLD_LIGHT
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        colors::TEXT
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        colors::GOLD_LIGHT
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        colors::TEXT
    }
}

impl checkbox::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: colors::BG.into(),
            border: Border {
                color: colors::GOLD,
                width: 1.0,
                radius: 5.0.into(),
            },
            icon_color: colors::GOLD,
            text_color: colors::GOLD.into(),
        }
    }

    fn disabled(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        let active = self.active(style, is_checked);

        let color = Color {
            a: active.icon_color.a * 0.5,
            ..active.icon_color
        };

        checkbox::Appearance {
            icon_color: color,
            text_color: color.into(),
            border: Border {
                color,
                ..active.border
            },
            ..active
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            background: colors::BG.into(),
            border: Border {
                color: colors::GOLD_LIGHT,
                width: 1.0,
                radius: 5.0.into(),
            },
            icon_color: colors::GOLD_LIGHT,
            text_color: colors::GOLD_LIGHT.into(),
        }
    }
}

impl rule::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
        rule::Appearance {
            color: colors::GOLD,
            width: 1,
            radius: 1.into(),
            fill_mode: rule::FillMode::Full,
        }
    }
}
