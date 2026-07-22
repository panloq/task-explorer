use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Alignment, Element, Length};
use crate::app::Message;
use crate::system::{format_bytes, format_rate, NetworkInterfaceInfo};
use crate::theme::ThemeColors;
use crate::lang::Lang;

pub fn render_network_view<'a>(networks: &'a [NetworkInterfaceInfo], lang: &'a Lang) -> Element<'a, Message> {
    let mut net_col = column![].spacing(16);

    for net in networks {
        let header = row![
            text("NET")
                .size(16)
                .style(|_| text::Style { color: Some(ThemeColors::NET_ACCENT) }),
            Space::with_width(Length::Fixed(12.0)),
            column![
                text(&net.name)
                    .size(16)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
                text(&lang.net_interface)
                    .size(12)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            ]
        ]
        .align_y(Alignment::Center);

        let rx_box = container(
            column![
                text(format!("{} (Download)", lang.net_download))
                    .size(11)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                Space::with_height(Length::Fixed(4.0)),
                text(format!("IN: {}", format_rate(net.rx_bytes_sec)))
                    .size(18)
                    .style(|_| text::Style { color: Some(ThemeColors::NET_ACCENT) }),
                Space::with_height(Length::Fixed(4.0)),
                text(format!("{}: {}", lang.net_total, format_bytes(net.total_rx_bytes)))
                    .size(11)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            ]
        )
        .padding(12)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(ThemeColors::CARD_HOVER)),
            border: iced::Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

        let tx_box = container(
            column![
                text(format!("{} (Upload)", lang.net_upload))
                    .size(11)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                Space::with_height(Length::Fixed(4.0)),
                text(format!("OUT: {}", format_rate(net.tx_bytes_sec)))
                    .size(18)
                    .style(|_| text::Style { color: Some(ThemeColors::CPU_ACCENT) }),
                Space::with_height(Length::Fixed(4.0)),
                text(format!("{}: {}", lang.net_total, format_bytes(net.total_tx_bytes)))
                    .size(11)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            ]
        )
        .padding(12)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(ThemeColors::CARD_HOVER)),
            border: iced::Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

        let rates_row = row![
            rx_box,
            Space::with_width(Length::Fixed(12.0)),
            tx_box
        ]
        .width(Length::Fill);

        let card = container(
            column![
                header,
                Space::with_height(Length::Fixed(16.0)),
                rates_row
            ]
        )
        .padding(20)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
            border: iced::Border {
                color: ThemeColors::CARD_BORDER,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        net_col = net_col.push(card);
    }

    let content = column![
        text(&lang.net_title)
            .size(20)
            .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
        Space::with_height(Length::Fixed(16.0)),
        net_col
    ]
    .padding(24);

    scrollable(content).into()
}
