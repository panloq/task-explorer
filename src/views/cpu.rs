use iced::widget::{column, container, row, text, Space};
use iced::{Alignment, Element, Length};
use crate::app::Message;
use crate::components::chart::cpu_chart_widget;
use crate::lang::Lang;
use crate::system::CpuInfo;
use crate::theme::ThemeColors;
use crate::components::progress_bar::custom_progress_bar;

pub fn render_cpu_view<'a>(
    cpu: &CpuInfo,
    cpu_history: &'a [f32],
    lang: &'a Lang,
) -> Element<'a, Message> {
    // Large chart section
    let chart_box = container(
        column![
            row![
                text(&lang.overview_cpu_chart).size(15)
                    .style(|_| text::Style { color: Some(ThemeColors::CPU_ACCENT) }),
                Space::with_width(Length::Fill),
                text(format!("{}: {:.1}%", lang.overview_now, cpu.global_usage)).size(15)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            ].align_y(Alignment::Center),
            Space::with_height(Length::Fixed(12.0)),
            cpu_chart_widget(cpu_history),
        ]
    )
    .padding(20).width(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
        border: iced::Border { color: ThemeColors::CPU_ACCENT, width: 1.0, radius: 8.0.into() },
        ..Default::default()
    });

    // Core list
    let mut core_grid = column![].spacing(8);
    let mut current_row = row![].spacing(12);
    for (idx, &usage) in cpu.cores.iter().enumerate() {
        let core_box = container(
            column![
                row![
                    text(format!("Core {}", idx)).size(11)
                        .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                    Space::with_width(Length::Fill),
                    text(format!("{:.0}%", usage)).size(11)
                        .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
                ],
                Space::with_height(Length::Fixed(4.0)),
                custom_progress_bar(usage, ThemeColors::CPU_ACCENT),
            ]
        )
        .padding(8).width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(ThemeColors::CARD_HOVER)),
            border: iced::Border { radius: 6.0.into(), ..Default::default() },
            ..Default::default()
        });
        current_row = current_row.push(core_box);
        if (idx + 1) % 4 == 0 || (idx + 1) == cpu.cores.len() {
            core_grid = core_grid.push(current_row);
            current_row = row![].spacing(12);
        }
    }

    let core_section = container(
        column![
            row![
                text(&lang.overview_cores).size(15)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
                Space::with_width(Length::Fill),
                text(cpu.brand.clone()).size(12)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            ].align_y(Alignment::Center),
            Space::with_height(Length::Fixed(12.0)),
            core_grid,
        ]
    )
    .padding(20).width(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
        border: iced::Border { color: ThemeColors::CARD_BORDER, width: 1.0, radius: 8.0.into() },
        ..Default::default()
    });

    let main_content = column![
        text("CPU Performance").size(24)
            .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
        Space::with_height(Length::Fixed(16.0)),
        chart_box,
        Space::with_height(Length::Fixed(16.0)),
        core_section,
    ]
    .padding(24);

    iced::widget::scrollable(main_content).into()
}
