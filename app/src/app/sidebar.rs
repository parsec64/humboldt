use iced::widget::{text, button, column, container, mouse_area, row, space};
use iced::{Alignment, Background, Border, Element, Length, Theme, color};
use iced_phosphor::regular;

use crate::app::{App, AppMessage};
use crate::data::settings::AppScreen;


impl<'a: 'static> App<'a> {
    pub fn sidebar_view(&self) -> Element<'_, AppMessage> {
        container(
            column![
                // Side bar close button
                row![
                    button(regular::list().size(20.0))
                        .on_press(AppMessage::SetSidebarOpen)
                        .padding([0, 15])
                        .style(|theme, _| button::Style {
                            text_color: theme.palette().primary,
                            ..Default::default()
                        }),

                    space()
                        .width(Length::Fill),
                ],

                // Shortcuts
                Self::screen_card_view(AppScreen::Explorer, self.screen == AppScreen::Explorer, &self.sidebar_open),
                Self::screen_card_view(AppScreen::Find, self.screen == AppScreen::Find, &self.sidebar_open),
                Self::screen_card_view(AppScreen::Home, self.screen == AppScreen::Home, &self.sidebar_open),
                Self::screen_card_view(AppScreen::Devices, self.screen == AppScreen::Devices, &self.sidebar_open),
                Self::screen_card_view(AppScreen::Explain, self.screen == AppScreen::Explain, &self.sidebar_open),
                Self::screen_card_view(AppScreen::Store, self.screen == AppScreen::Store, &self.sidebar_open),

                space().height(Length::Fill),
                Self::screen_card_view(AppScreen::Settings, self.screen == AppScreen::Settings, &self.sidebar_open),
            ]
            .spacing(10)
        )
        .height(Length::Fill)
        .width(match self.sidebar_open {
            true => 180.0,
            false => 70.0,
        })
        .padding(10)
        .style(|theme: &Theme| container::Style {
            border: Border {
                color: theme.palette().primary,
                width: 0.5,
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    fn screen_card_view<'b>(screen: AppScreen, selected: bool, open: &bool) -> Element<'static, AppMessage> {
        mouse_area(
            button(
                container(
                    row![
                        // Icon
                        match screen {
                            AppScreen::Explorer => regular::desktop().color(color!(0xffc869)),
                            AppScreen::Find => regular::magnifying_glass().color(color!(0xfcf3e3)),
                            AppScreen::Home => regular::house().color(color!(0xdedddc)),
                            AppScreen::Devices => regular::hard_drive().color(color!(0xdedddc)),
                            AppScreen::Explain => regular::book().color(color!(0x827865)),
                            AppScreen::Store => regular::storefront().color(color!(0x5d63c7)),
                            AppScreen::Settings => regular::gear_six().color(color!(0x3d3d3d)),
                        }
                        .size(20),

                        space()
                            .width(20.0),

                        // Name
                        match open {
                            true => text(
                                match screen {
                                    AppScreen::Explorer => "Explorer",
                                    AppScreen::Find => "Find",
                                    AppScreen::Home => "Home",
                                    AppScreen::Devices => "Devices",
                                    AppScreen::Explain => "Explain",
                                    AppScreen::Store => "Store",
                                    AppScreen::Settings => "Settings",
                                }
                            )
                            .style(|theme: &Theme| text::Style {
                                color: Some(theme.palette().text),
                                ..Default::default()
                            }),
                            false => text(""),
                        },

                        space()
                            .width(Length::Fill),
                    ]
                    .align_y(Alignment::Center)
                )
                .width(Length::Fill)
                .align_x(Alignment::Center)
            )
            .on_press(AppMessage::SetScreen(screen))
            .width(Length::Fill)
            .padding([8, 15])
            .style(move |theme, status| button::Style {
                background: if selected {
                    Some(Background::Color(theme.palette().primary.scale_alpha(0.1)))
                } else {
                    match status {
                        button::Status::Hovered | button::Status::Pressed =>
                            Some(Background::Color(theme.palette().primary.scale_alpha(0.1))),
                        _ => None,
                    }
                },
                border: Border {
                    radius: 5.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
        )
        .into()
    }
}
