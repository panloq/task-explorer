use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Color, Element, Length};
use crate::app::Message;
use crate::lang::Lang;
use crate::system::{format_bytes, format_duration, format_rate, ProcessItem};
use crate::theme::ThemeColors;

/// Modal overlay for process details. Renders on top of the existing view.
pub fn render_process_info_modal<'a>(
    proc: &'a ProcessItem,
    is_suspended: bool,
    lang: &'a Lang,
) -> Element<'a, Message> {
    let age = proc.age_secs();
    let age_str = if age == 0 {
        lang.unknown.to_string()
    } else {
        format_duration(age)
    };

    let parent_str = proc.parent_pid
        .map(|p| p.to_string())
        .unwrap_or_else(|| lang.none.to_string());

    let path_str = if proc.exe_path.is_empty() {
        lang.unknown.to_string()
    } else {
        proc.exe_path.clone()
    };

    let cmd_str = if proc.cmd_line.is_empty() {
        lang.none.to_string()
    } else {
        proc.cmd_line.clone()
    };

    fn label<'a>(t: &'a str) -> iced::widget::Text<'a> {
        text(t).size(12).style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) })
    }

    fn value<'a>(t: String) -> iced::widget::Text<'a> {
        text(t).size(13).style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) })
    }

    fn info_row<'a>(lbl: &'a str, val: String) -> Element<'a, Message> {
        row![
            container(label(lbl)).width(Length::Fixed(150.0)),
            value(val),
        ]
        .align_y(Alignment::Start)
        .spacing(8)
        .into()
    }

    fn divider<'a>() -> Element<'a, Message> {
        container(Space::with_height(Length::Fixed(1.0)))
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(ThemeColors::CARD_BORDER)),
                ..Default::default()
            })
            .into()
    }

    let zombie_warning: Element<Message> = if proc.is_zombie_like() {
        container(
            text(lang.zombie_tooltip.clone())
                .size(12)
                .style(|_| text::Style { color: Some(ThemeColors::DANGER) })
        )
        .padding([8, 12])
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgba(0.8, 0.1, 0.1, 0.15))),
            border: iced::Border {
                color: ThemeColors::DANGER,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .into()
    } else {
        Space::with_height(Length::Fixed(0.0)).into()
    };

    let suspend_btn = if is_suspended {
        button(
            text(lang.proc_resume.clone())
                .size(12)
                .style(|_| text::Style { color: Some(Color::WHITE) })
        )
        .padding([10, 16])
        .on_press(Message::ResumeProcess(proc.pid))
        .style(|_, status| button::Style {
            background: Some(iced::Background::Color(if status == button::Status::Hovered {
                Color::from_rgb(0.2, 0.7, 0.3)
            } else {
                Color::from_rgb(0.1, 0.6, 0.2)
            })),
            text_color: Color::WHITE,
            border: iced::Border { radius: 6.0.into(), ..Default::default() },
            shadow: Default::default(),
        })
    } else {
        button(
            text(lang.proc_suspend.clone())
                .size(12)
                .style(|_| text::Style { color: Some(Color::WHITE) })
        )
        .padding([10, 16])
        .on_press(Message::SuspendProcess(proc.pid))
        .style(|_, status| button::Style {
            background: Some(iced::Background::Color(if status == button::Status::Hovered {
                Color::from_rgb(0.9, 0.6, 0.1)
            } else {
                Color::from_rgb(0.8, 0.5, 0.0)
            })),
            text_color: Color::WHITE,
            border: iced::Border { radius: 6.0.into(), ..Default::default() },
            shadow: Default::default(),
        })
    };

    let kill_tree_btn = button(
        text(lang.proc_kill_tree.clone())
            .size(12)
            .style(|_| text::Style { color: Some(Color::WHITE) })
    )
    .padding([10, 16])
    .on_press(Message::KillProcessTree(proc.pid))
    .style(|_, status| button::Style {
        background: Some(iced::Background::Color(if status == button::Status::Hovered {
            Color::from_rgb(0.7, 0.1, 0.3)
        } else {
            Color::from_rgb(0.6, 0.0, 0.2)
        })),
        text_color: Color::WHITE,
        border: iced::Border { radius: 6.0.into(), ..Default::default() },
        shadow: Default::default(),
    });

    let modal_content = column![
        // Title bar
        row![
            text(lang.info_title.clone())
                .size(18)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            Space::with_width(Length::Fill),
            button(
                text(lang.info_close.clone())
                    .size(12)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) })
            )
            .padding([6, 14])
            .on_press(Message::CloseProcessInfo)
            .style(|_, status| button::Style {
                background: Some(iced::Background::Color(if status == button::Status::Hovered {
                    ThemeColors::CARD_HOVER
                } else {
                    Color::TRANSPARENT
                })),
                text_color: ThemeColors::TEXT_MUTED,
                border: iced::Border {
                    color: ThemeColors::CARD_BORDER,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Default::default(),
            }),
        ]
        .align_y(Alignment::Center),

        Space::with_height(Length::Fixed(4.0)),
        divider(),
        Space::with_height(Length::Fixed(12.0)),

        zombie_warning,
        Space::with_height(Length::Fixed(8.0)),

        // --- Identity section ---
        text("Identity")
            .size(11)
            .style(|_| text::Style { color: Some(ThemeColors::CPU_ACCENT) }),
        Space::with_height(Length::Fixed(6.0)),
        info_row(&lang.info_pid, proc.pid.to_string()),
        info_row(&lang.info_name, proc.name.clone()),
        info_row(&lang.info_status, proc.status.clone()),
        info_row(&lang.info_age, age_str),
        info_row(&lang.info_parent, parent_str),

        Space::with_height(Length::Fixed(10.0)),
        divider(),
        Space::with_height(Length::Fixed(10.0)),

        // --- Resource usage section ---
        text("Resource Usage")
            .size(11)
            .style(|_| text::Style { color: Some(ThemeColors::RAM_ACCENT) }),
        Space::with_height(Length::Fixed(6.0)),
        info_row(&lang.info_cpu, format!("{:.2}%", proc.cpu_usage)),
        info_row(&lang.info_ram, format_bytes(proc.memory_bytes)),
        info_row(&lang.info_disk_read, format_rate(proc.disk_read_bytes_sec)),
        info_row(&lang.info_disk_write, format_rate(proc.disk_write_bytes_sec)),
        info_row(&lang.info_total_read, format_bytes(proc.total_read_bytes)),
        info_row(&lang.info_total_written, format_bytes(proc.total_written_bytes)),

        Space::with_height(Length::Fixed(10.0)),
        divider(),
        Space::with_height(Length::Fixed(10.0)),

        // --- Path / command section ---
        text("File Info")
            .size(11)
            .style(|_| text::Style { color: Some(ThemeColors::DISK_ACCENT) }),
        Space::with_height(Length::Fixed(6.0)),
        info_row(&lang.info_path, path_str),
        info_row(&lang.info_cmd, cmd_str),

        Space::with_height(Length::Fixed(16.0)),

        // Action buttons
        row![
            button(
                text(lang.info_kill.clone())
                    .size(12)
                    .style(|_| text::Style { color: Some(Color::WHITE) })
            )
            .padding([10, 16])
            .on_press(Message::KillProcess(proc.pid))
            .style(|_, status| button::Style {
                background: Some(iced::Background::Color(if status == button::Status::Hovered {
                    Color::from_rgb(0.85, 0.2, 0.2)
                } else {
                    ThemeColors::DANGER
                })),
                text_color: Color::WHITE,
                border: iced::Border { radius: 6.0.into(), ..Default::default() },
                shadow: Default::default(),
            }),
            Space::with_width(Length::Fixed(8.0)),
            kill_tree_btn,
            Space::with_width(Length::Fixed(8.0)),
            suspend_btn,
            Space::with_width(Length::Fixed(8.0)),
            button(
                text(lang.info_close.clone())
                    .size(12)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) })
            )
            .padding([10, 16])
            .on_press(Message::CloseProcessInfo)
            .style(|_, status| button::Style {
                background: Some(iced::Background::Color(if status == button::Status::Hovered {
                    ThemeColors::CARD_HOVER
                } else {
                    ThemeColors::CARD_BG
                })),
                text_color: ThemeColors::TEXT_MUTED,
                border: iced::Border {
                    color: ThemeColors::CARD_BORDER,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Default::default(),
            }),
        ]
        .align_y(Alignment::Center),
    ]
    .spacing(4)
    .padding(24)
    .width(Length::Fixed(560.0));

    let modal_card = container(scrollable(modal_content))
        .max_height(600)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(ThemeColors::SIDEBAR_BG)),
            text_color: None,
            border: iced::Border {
                color: ThemeColors::CPU_ACCENT,
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.6),
                offset: iced::Vector::new(0.0, 8.0),
                blur_radius: 32.0,
            },
        });

    // Backdrop + centered card
    container(
        container(modal_card)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.6))),
        ..Default::default()
    })
    .into()
}
