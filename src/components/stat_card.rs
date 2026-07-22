use iced::widget::{button, column, container, row, text, Space};
use iced::{Color, Element, Length, Alignment};
use crate::app::{Message, Tab};
use crate::components::progress_bar::custom_progress_bar;
use crate::theme::ThemeColors;

pub fn render_stat_card<'a>(
    title: &'a str,
    value: String,
    subtitle: String,
    percent: Option<f32>,
    accent_color: Color,
    target_tab: Tab,
) -> Element<'a, Message> {
    let header = row![
        text(title)
            .size(13)
            .style(move |_| text::Style {
                color: Some(ThemeColors::TEXT_MUTED),
            }),
        Space::with_width(Length::Fill),
        container(text(""))
            .width(Length::Fixed(10.0))
            .height(Length::Fixed(10.0))
            .style(move |_| container::Style {
                background: Some(iced::Background::Color(accent_color)),
                border: iced::Border {
                    radius: 5.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
    ]
    .align_y(Alignment::Center);

    let val_text = text(value)
        .size(24)
        .style(move |_| text::Style {
            color: Some(ThemeColors::TEXT_PRIMARY),
        });

    let sub_text = text(subtitle)
        .size(11)
        .style(move |_| text::Style {
            color: Some(ThemeColors::TEXT_MUTED),
        });

    let mut content = column![header, Space::with_height(Length::Fixed(6.0)), val_text, sub_text];

    if let Some(p) = percent {
        content = content
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(custom_progress_bar(p, accent_color));
    }

    button(
        container(content)
            .padding(16)
            .width(Length::Fill)
            .style(|_| container::Style::default())
    )
    .on_press(Message::TabSelected(target_tab))
    .width(Length::Fill)
    .style(move |_, status| button::Style {
        background: Some(iced::Background::Color(if status == button::Status::Hovered || status == button::Status::Pressed {
            // Brighten card background when hovered or clicked
            Color::from_rgb(0.18, 0.23, 0.32)
        } else {
            ThemeColors::CARD_BG
        })),
        text_color: ThemeColors::TEXT_PRIMARY,
        border: iced::Border {
            color: if status == button::Status::Hovered {
                Color::from_rgb(0.3, 0.45, 0.7)
            } else {
                ThemeColors::CARD_BORDER
            },
            width: 1.0,
            radius: 8.0.into(),
        },
        shadow: Default::default(),
    })
    .into()
}
