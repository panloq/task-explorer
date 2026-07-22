use iced::widget::{column, container, row, text, Space};
use iced::{Alignment, Element, Length};
use crate::app::Message;
use crate::components::chart::ram_chart_widget;
use crate::lang::Lang;
use crate::system::{MemoryInfo, format_bytes};
use crate::theme::ThemeColors;

pub fn render_ram_view<'a>(
    mem: &MemoryInfo,
    memory_history: &'a [f32],
    lang: &'a Lang,
) -> Element<'a, Message> {
    // Large chart section
    let chart_box = container(
        column![
            row![
                text(&lang.overview_ram_chart).size(15)
                    .style(|_| text::Style { color: Some(ThemeColors::RAM_ACCENT) }),
                Space::with_width(Length::Fill),
                text(format!("{}: {:.1}%", lang.overview_now, mem.used_percent())).size(15)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            ].align_y(Alignment::Center),
            Space::with_height(Length::Fixed(12.0)),
            ram_chart_widget(memory_history),
        ]
    )
    .padding(20).width(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
        border: iced::Border { color: ThemeColors::RAM_ACCENT, width: 1.0, radius: 8.0.into() },
        ..Default::default()
    });

    // RAM details section
    let details_section = container(
        column![
            text("Memory Details").size(16)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            Space::with_height(Length::Fixed(12.0)),
            row![
                text("Used RAM: ").style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                text(format_bytes(mem.used_bytes)).style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            ],
            Space::with_height(Length::Fixed(8.0)),
            row![
                text("Total RAM: ").style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                text(format_bytes(mem.total_bytes)).style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            ],
            Space::with_height(Length::Fixed(8.0)),
            row![
                text("Used Swap: ").style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                text(format_bytes(mem.swap_used_bytes)).style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            ],
            Space::with_height(Length::Fixed(8.0)),
            row![
                text("Total Swap: ").style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                text(format_bytes(mem.swap_total_bytes)).style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            ],
        ]
    )
    .padding(20).width(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
        border: iced::Border { color: ThemeColors::CARD_BORDER, width: 1.0, radius: 8.0.into() },
        ..Default::default()
    });

    let main_content = column![
        text("RAM Performance").size(24)
            .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
        Space::with_height(Length::Fixed(16.0)),
        chart_box,
        Space::with_height(Length::Fixed(16.0)),
        details_section,
    ]
    .padding(24);

    iced::widget::scrollable(main_content).into()
}
