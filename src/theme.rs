use crate::colors;
use iced::widget::{button, checkbox, container, rule, text, text_input};
use iced::{application, Background, Border, Color};

#[derive(Debug, Clone, Copy, Default)]
pub struct Theme;

impl iced::application::DefaultStyle for Theme {
    fn default_style(&self) -> application::Appearance {
        application::Appearance {
            text_color: colors::TEXT,
            background_color: colors::BG,
        }
    }
}

// impl application::Catalog for Theme {
//     type Class<'a> = ();
//
//     fn appearance(&self, _style: &Self::Style) -> application::Style {
//             text_color: colors::TEXT,
//         }
//     }
// }

#[derive(Debug, Clone, Copy, Default)]
pub enum Text {
    #[default]
    Default,
    Error,
    Success,
}

impl iced::widget::text::Catalog for Theme {
    type Class<'a> = Text;

    fn default<'a>() -> Self::Class<'a> {
        Text::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        let color = match class {
            // Text::Error => colors::RED,
            // Text::Success => colors::GREEN,
            _ => colors::TEXT,
        }
        .into();
        text::Style { color }
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

impl container::Catalog for Theme {
    type Class<'a> = Container;

    fn default<'a>() -> Self::Class<'a> {
        Container::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        match class {
            Container::Error => container::Style {
                background: Background::Color(colors::RED).into(),
                border: Border {
                    color: colors::RED_BG,
                    width: 2.0,
                    radius: 5.into(),
                },
                text_color: colors::RED.into(),
                ..container::Style::default()
            },
            Container::Success => container::Style {
                background: Background::Color(colors::GREEN).into(),
                border: Border {
                    color: colors::GREEN_BG,
                    width: 2.0,
                    radius: 5.into(),
                },
                text_color: colors::GREEN.into(),
                ..container::Style::default()
            },
            Container::Tooltip => container::Style {
                background: Background::Color(colors::BG).into(),
                border: Border {
                    color: colors::BG,
                    width: 1.0,
                    radius: 10.into(),
                },

                text_color: colors::TEXT.into(),
                ..container::Style::default()
            },

            Container::Default => container::Style::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Primary,
    Fill(Color),
}

impl button::Catalog for Theme {
    type Class<'a> = Button;

    fn default<'a>() -> Self::Class<'a> {
        Button::Primary
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        let mut color = match class {
            Button::Fill(colors::GOLD) => {
                if status == button::Status::Active {
                    colors::GOLD
                } else {
                    colors::GOLD_LIGHT
                }
            }
            Button::Fill(colors::GREEN) => {
                if status == button::Status::Active {
                    colors::GREEN
                } else {
                    colors::GREEN_BG
                }
            }

            Button::Fill(colors::RED) => {
                if status == button::Status::Active {
                    colors::RED
                } else {
                    colors::RED_BG
                }
            }
            _ => {
                if status == button::Status::Active {
                    colors::BLUE
                } else {
                    colors::BLUE_DARK
                }
            }
        };

        if status == button::Status::Disabled {
            let col = self.style(class, button::Status::Active).border.color;
            color = Color {
                a: col.a * 0.5,
                ..col
            };
        }

        button::Style {
            background: Background::Color(color).into(),
            border: Border {
                color,
                width: 5.0,
                radius: 5.into(),
            },
            text_color: colors::TEXT,
            ..button::Style::default()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TextInput {
    #[default]
    Default,
}

impl text_input::Catalog for Theme {
    type Class<'a> = TextInput;

    // fn active(&self, style: &Self::Style) -> text_input::Style {
    //     match style {
    //         TextInput::Default => text_input::Style {
    //             background: colors::BG.into(),
    //             border: Border {
    //                 color: colors::TEXT,
    //                 width: 1.0,
    //                 radius: 5.into(),
    //             },
    //             icon_color: colors::TEXT,
    //         },
    //     }
    // }
    //
    // fn disabled(&self, style: &Self::Style) -> text_input::Style {
    //     match style {
    //         TextInput::Default => text_input::Style {
    //             background: colors::BG.into(),
    //             border: Border {
    //                 color: colors::GOLD,
    //                 width: 2.0,
    //                 radius: 5.into(),
    //             },
    //             icon_color: colors::GOLD_LIGHT,
    //         },
    //     }
    // }

    // fn focused(&self, style: &Self::Style) -> text_input::Style {
    //     self.active(style)
    // }
    //
    // fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
    //     colors::GOLD_LIGHT
    // }
    //
    // fn value_color(&self, _style: &Self::Style) -> iced::Color {
    //     colors::TEXT
    // }
    //
    // fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
    //     colors::GOLD_LIGHT
    // }
    //
    // fn selection_color(&self, _style: &Self::Style) -> iced::Color {
    //     colors::TEXT
    // }

    fn default<'a>() -> Self::Class<'a> {
        TextInput::Default
    }

    fn style(&self, _class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        let flag = (status == text_input::Status::Disabled);
        let text_color = if flag {
            colors::GOLD_LIGHT
        } else {
            colors::TEXT
        };

        let border_color = if flag { colors::GOLD } else { colors::TEXT };

        text_input::Style {
            background: colors::BG.into(),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: 5.into(),
            },
            placeholder: text_color,
            icon: text_color,
            value: text_color,
            selection: text_color,
        }
    }
}

impl checkbox::Catalog for Theme {
    type Class<'a> = ();

    // fn active(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Style {
    //     checkbox::Style {
    //         background: colors::BG.into(),
    //         border: Border {
    //             color: colors::GOLD,
    //             width: 1.0,
    //             radius: 5.0.into(),
    //         },
    //         icon_color: colors::GOLD,
    //         text_color: colors::GOLD.into(),
    //     }
    // }
    //
    // fn disabled(&self, style: &Self::Style, is_checked: bool) -> checkbox::Style {
    //     let active = self.active(style, is_checked);
    //
    //     let color = Color {
    //         a: active.icon_color.a * 0.5,
    //         ..active.icon_color
    //     };
    //
    //     checkbox::Style {
    //         icon_color: color,
    //         text_color: color.into(),
    //         border: Border {
    //             color,
    //             ..active.border
    //         },
    //         ..active
    //     }
    // }
    //
    // fn hovered(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Style {
    //     checkbox::Style {
    //         background: colors::BG.into(),
    //         border: Border {
    //             color: colors::GOLD_LIGHT,
    //             width: 1.0,
    //             radius: 5.0.into(),
    //         },
    //         icon_color: colors::GOLD_LIGHT,
    //         text_color: colors::GOLD_LIGHT.into(),
    //     }
    // }

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>, status: checkbox::Status) -> checkbox::Style {
        let color = match status {
            checkbox::Status::Active { .. } => colors::GOLD,
            checkbox::Status::Disabled { .. } => Color {
                a: colors::GOLD.a * 0.5,
                ..colors::GOLD
            },
            checkbox::Status::Hovered { .. } => colors::GOLD_LIGHT,
        };

        checkbox::Style {
            background: colors::BG.into(),
            border: Border {
                color,
                width: 1.0,
                radius: 5.0.into(),
            },
            icon_color: color,
            text_color: color.into(),
        }
    }
}

impl rule::Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(&self, _class: &Self::Class<'_>) -> rule::Style {
        rule::Style {
            color: colors::GOLD,
            width: 1,
            radius: 1.into(),
            fill_mode: rule::FillMode::Full,
        }
    }

    // fn style(&self, _style: &Self::Style) -> rule::Style {
    //     rule::Style {
    //         color: colors::GOLD,
    //         width: 1,
    //         radius: 1.into(),
    //         fill_mode: rule::FillMode::Full,
    //     }
    // }
}
