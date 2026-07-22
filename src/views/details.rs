use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Color, Element, Length};
use crate::app::{Message, SortKey};
use crate::lang::Lang;
use crate::system::{format_bytes, ProcessItem};
use crate::theme::ThemeColors;

pub fn render_details_view<'a>(
    processes: &'a [ProcessItem],
    search_query: &str,
    sort_key: SortKey,
    sort_ascending: bool,
    selected_pid: Option<u32>,
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

    let selected_proc = selected_pid.and_then(|pid| processes.iter().find(|p| p.pid == pid));

    // Action bar
    let mut actions = row![].spacing(8).align_y(Alignment::Center);

    if let Some(proc) = selected_proc {
        let pid = proc.pid;
        let name = proc.name.clone();
        let path = proc.exe_path.clone();

        actions = actions
            .push(
                button(text(format!("{} (PID: {})", lang.proc_info, pid)).size(12)
                    .style(|_| text::Style { color: Some(Color::WHITE) }))
                .padding([8, 14])
                .on_press(Message::OpenProcessInfo(pid))
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
                button(text(&lang.startup_open_location).size(12)
                    .style(|_| text::Style { color: Some(Color::WHITE) }))
                .padding([8, 14])
                .on_press(Message::OpenStartupLocation(if path.is_empty() { name.clone() } else { path }))
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
                button(text(format!("{} ({})", lang.proc_kill, pid)).size(12)
                    .style(|_| text::Style { color: Some(Color::WHITE) }))
                .padding([8, 14])
                .on_press(Message::KillProcess(pid))
                .style(|_, status| button::Style {
                    background: Some(iced::Background::Color(if status == button::Status::Hovered {
                        Color::from_rgb(0.8, 0.2, 0.2)
                    } else {
                        ThemeColors::DANGER
                    })),
                    text_color: Color::WHITE,
                    border: iced::Border { radius: 6.0.into(), ..Default::default() },
                    shadow: Default::default(),
                })
            );
    }

    // Header buttons
    fn header_btn<'a>(label: &'a str, key: SortKey, current: SortKey, asc: bool) -> Element<'a, Message> {
        let active = current == key;
        let indicator = if active { if asc { " [^]" } else { " [v]" } } else { "" };
        button(
            text(format!("{}{}", label, indicator)).size(12)
                .style(move |_| text::Style {
                    color: Some(if active { ThemeColors::CPU_ACCENT } else { ThemeColors::TEXT_MUTED }),
                })
        )
        .on_press(Message::SortBy(key))
        .padding([6, 8])
        .style(|_, _| button::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            text_color: ThemeColors::TEXT_MUTED,
            border: Default::default(),
            shadow: Default::default(),
        })
        .into()
    }

    let header_row = container(
        row![
            container(header_btn(&lang.proc_col_pid, SortKey::Pid, sort_key, sort_ascending)).width(Length::Fixed(72.0)),
            container(header_btn(&lang.proc_col_name, SortKey::Name, sort_key, sort_ascending)).width(Length::Fixed(160.0)),
            container(header_btn(&lang.proc_col_cpu, SortKey::Cpu, sort_key, sort_ascending)).width(Length::Fixed(80.0)),
            container(header_btn(&lang.proc_col_ram, SortKey::Memory, sort_key, sort_ascending)).width(Length::Fixed(100.0)),
            text(&lang.info_path).size(12).width(Length::Fill)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            text(&lang.proc_col_status).size(12).width(Length::Fixed(90.0))
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

    for proc in processes {
        let is_selected = selected_pid == Some(proc.pid);
        let pid_val = proc.pid;
        let path_display = if proc.exe_path.is_empty() { "-" } else { &proc.exe_path };

        let row_content = row![
            text(proc.pid.to_string()).size(12).width(Length::Fixed(72.0))
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            text(&proc.name).size(12).width(Length::Fixed(160.0))
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            text(format!("{:.1}%", proc.cpu_usage)).size(12).width(Length::Fixed(80.0))
                .style(|_| text::Style { color: Some(ThemeColors::CPU_ACCENT) }),
            text(format_bytes(proc.memory_bytes)).size(12).width(Length::Fixed(100.0))
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            text(path_display).size(11).width(Length::Fill)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            text(&proc.status).size(11).width(Length::Fixed(90.0))
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
        ]
        .align_y(Alignment::Center)
        .padding([6, 12]);

        let row_btn = button(row_content)
            .width(Length::Fill)
            .on_press(Message::SelectProcess(if is_selected { None } else { Some(pid_val) }))
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
            text(format!("{}: {}", lang.proc_count, processes.len())).size(13)
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
