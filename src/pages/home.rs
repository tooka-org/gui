use std::path::PathBuf;

use crate::{
    app::{Message, Tooka},
    theme::{button::{button_primary_style, button_transparent_style}, container::{card_container_style}, icon::white_icon_style, svg::svg_icon, PRIMARY_COLOR, SECONDARY_COLOR},
};

impl Tooka {
    pub fn home(&self) -> iced::Element<Message> {
        let nav_bar = iced::widget::Row::new()
            .push(
                iced::widget::container(
                    iced::widget::text("Tooka")
                        .size(35)
                        .font(iced::font::Font {
                            weight: iced::font::Weight::Bold,
                            ..iced::font::Font::DEFAULT
                        })
                        .style(|_| iced::widget::text::Style {
                            color: Some(PRIMARY_COLOR),
                        }),
                )
                .padding([5, 0]),
            )
            .push(iced::widget::horizontal_space())
            .push(
                iced::widget::Button::new(svg_icon(crate::LICENSE).style(white_icon_style))
                    .on_press(Message::OpenRules)
                    .style(button_primary_style)
                    .padding(10),
            )
            .push(iced::widget::Space::with_width(5.0))
            .push(
                iced::widget::Button::new(svg_icon(crate::SETTINGS).style(white_icon_style))
                    .on_press(Message::OpenSettings)
                    .style(button_primary_style)
                    .padding(10),
            )
            .padding(10);

        let center_logo = iced::widget::container(iced::widget::image(iced::widget::image::Handle::from_bytes(crate::LOGO))
            .width(200)
            .height(200)
            .content_fit(iced::ContentFit::Cover)
        ).center(iced::Length::Fill)
        .padding(10)
        .height(200)
        .width(200);
        

        let center_input = iced::widget::Column::new()
            .push(
                iced::widget::Row::new()
                    .push(iced::widget::container(iced::widget::text("Selected folder").size(10).style(|_| {
                        iced::widget::text::Style {
                            color: Some(SECONDARY_COLOR),
                        }
                    }))
                    .style(card_container_style)
                )
                    .push(
                        iced::widget::Button::new(svg_icon(crate::FOLDER).style(white_icon_style))
                            .on_press(Message::Open {
                                path: PathBuf::new(),
                            })
                            .style(button_primary_style)
                            .padding(5),
                    )
                    .spacing(5.0)
                    .align_y(iced::Alignment::Center)
                    .padding(10)
            )
            .push(
                iced::widget::Row::new()
                    .push(
                        iced::widget::Button::new(iced::widget::text("SORT"))
                            .on_press(Message::StartSorting)
                            .style(button_primary_style)
                            .padding(5),
                    )
                    .push(iced::widget::checkbox("Dry Run", false))
                    .spacing(10.0)
                    .align_y(iced::Alignment::Center)
            )
            .padding(10)
            .align_x(iced::Alignment::Center)
            .width(iced::Length::Fill);

        let bottom_text = iced::widget::container(iced::widget::Button::new(iced::widget::text("Terms & Conditions"))
            .on_press(Message::OpenTerms)
            .style(button_transparent_style)
            .padding(5),
        ).center(iced::Length::Fill);

        iced::widget::Column::new()
            .push(nav_bar)
            .push(center_logo)
            .push(center_input)
            .push(bottom_text)
            .spacing(5)
            .align_x(iced::Alignment::Center)
            .into()
    }
}
