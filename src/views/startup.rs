use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Color, Element, Length};
use crate::app::Message;
use crate::lang::Lang;
use crate::startup::StartupItem;
use crate::theme::ThemeColors;

pub fn render_startup_view<'a>(
    items: &'a [StartupItem],
    search_query: &str,
    selected_index: Option<usize>,
    lang: &'a Lang,
) -> Element<'a, Message> {
    let search_input = text_input(&lang.proc_search_placeholder, search_query)
        .on_input(Message::SearchQueryChanged)
        .padding(10)
        .size(13)
        .style(|theme, status| {
            let mut style = text_input::default(theme, status);
            style.background = iced::Background::Color(ThemeColors::CARD_BG);
            style.value = ThemeColors::TEXT_PRIMARY;
            style.placeholder = ThemeColors::TEXT_MUTED;
            style.border = iced::Border {
                color: ThemeColors::CARD_BORDER,
                width: 1.0,
                radius: 6.0.into(),
            };
            style
        });

    let q = search_query.trim().to_lowercase();
    let filtered_items: Vec<(usize, &StartupItem)> = items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            q.is_empty()
                || item.name.to_lowercase().contains(&q)
                || item.command.to_lowercase().contains(&q)
                || item.location.to_lowercase().contains(&q)
        })
        .collect();

    // Actions bar
    let mut actions = row![].spacing(8).align_y(Alignment::Center);

    if let Some(idx) = selected_index {
        if let Some(item) = items.get(idx) {
            let cmd = item.command.clone();
            let name = item.name.clone();

            actions = actions
                .push(
                    button(text(&lang.startup_open_location).size(12)
                        .style(|_| text::Style { color: Some(Color::WHITE) }))
                    .padding([8, 14])
                    .on_press(Message::OpenStartupLocation(cmd.clone()))
                    .style(|_, status| button::Style {
                        background: Some(iced::Background::Color(if status == button::Status::Hovered {
                            Color::from_rgb(0.2, 0.6, 0.8)
                        } else {
                            Color::from_rgb(0.1, 0.5, 0.7)
                        })),
                        text_color: Color::WHITE,
                        border: iced::Border { radius: 6.0.into(), ..Default::default() },
                        shadow: Default::default(),
                    })
                )
                .push(
                    button(text(&lang.proc_search_online).size(12)
                        .style(|_| text::Style { color: Some(Color::WHITE) }))
                    .padding([8, 14])
                    .on_press(Message::SearchProcessOnline(name))
                    .style(|_, status| button::Style {
                        background: Some(iced::Background::Color(if status == button::Status::Hovered {
                            Color::from_rgb(0.3, 0.5, 0.9)
                        } else {
                            Color::from_rgb(0.2, 0.4, 0.8)
                        })),
                        text_color: Color::WHITE,
                        border: iced::Border { radius: 6.0.into(), ..Default::default() },
                        shadow: Default::default(),
                    })
                )
                .push(
                    button(text(&lang.startup_remove).size(12)
                        .style(|_| text::Style { color: Some(Color::WHITE) }))
                    .padding([8, 14])
                    .on_press(Message::RemoveStartupItem(idx))
                    .style(|_, status| button::Style {
                        background: Some(iced::Background::Color(if status == button::Status::Hovered {
                            Color::from_rgb(0.9, 0.25, 0.25)
                        } else {
                            Color::from_rgb(0.8, 0.2, 0.2)
                        })),
                        text_color: Color::WHITE,
                        border: iced::Border { radius: 6.0.into(), ..Default::default() },
                        shadow: Default::default(),
                    })
                );
        }
    }

    let header_row = container(
        row![
            text(&lang.startup_col_name).size(12).width(Length::Fixed(180.0))
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            text(&lang.startup_col_location).size(12).width(Length::Fixed(160.0))
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            text(&lang.startup_col_cmd).size(12).width(Length::Fill)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            text(&lang.startup_col_status).size(12).width(Length::Fixed(90.0))
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
        ]
        .align_y(Alignment::Center)
    )
    .padding([8, 12])
    .style(|_| container::Style {
        background: Some(iced::Background::Color(ThemeColors::SIDEBAR_BG)),
        border: iced::Border {
            color: ThemeColors::CARD_BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    });

    let mut rows_col = column![].spacing(2);

    for (orig_idx, item) in filtered_items {
        let is_selected = selected_index == Some(orig_idx);
        let status_label = if item.enabled { &lang.startup_enabled } else { &lang.startup_disabled };
        let status_color = if item.enabled { ThemeColors::NET_ACCENT } else { ThemeColors::TEXT_MUTED };

        let row_content = row![
            text(&item.name).size(12).width(Length::Fixed(180.0))
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            text(&item.location).size(11).width(Length::Fixed(160.0))
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            text(&item.command).size(11).width(Length::Fill)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            text(status_label).size(11).width(Length::Fixed(90.0))
                .style(move |_| text::Style { color: Some(status_color) }),
        ]
        .align_y(Alignment::Center)
        .padding([8, 12]);

        let row_btn = button(row_content)
            .width(Length::Fill)
            .on_press(Message::SelectStartupItem(if is_selected { None } else { Some(orig_idx) }))
            .style(move |_, status| button::Style {
                background: Some(iced::Background::Color(if is_selected {
                    Color::from_rgba(0.02, 0.71, 0.83, 0.15)
                } else if status == button::Status::Hovered {
                    ThemeColors::CARD_HOVER
                } else {
                    ThemeColors::CARD_BG
                })),
                text_color: ThemeColors::TEXT_PRIMARY,
                border: iced::Border {
                    color: if is_selected { ThemeColors::CPU_ACCENT } else { Color::TRANSPARENT },
                    width: if is_selected { 1.0 } else { 0.0 },
                    radius: 4.0.into(),
                },
                shadow: Default::default(),
            });

        rows_col = rows_col.push(row_btn);
    }

    let scrollable_table = scrollable(rows_col).height(Length::Fill);

    let content = column![
        search_input,
        Space::with_height(Length::Fixed(12.0)),
        row![
            text(format!("{}: {}", lang.proc_count, items.len())).size(13)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            Space::with_width(Length::Fill),
            actions,
        ].align_y(Alignment::Center),
        Space::with_height(Length::Fixed(12.0)),
        header_row,
        Space::with_height(Length::Fixed(6.0)),
        scrollable_table,
    ];

    container(content).into()
}
