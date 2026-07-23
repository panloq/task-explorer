use iced::widget::{button, column, container, image, row, scrollable, text, text_input, Space};
use iced::{Alignment, Color, Element, Length};
use crate::app::Message;
use crate::app::SortKey;
use crate::lang::Lang;
use crate::system::{format_bytes, format_duration, format_rate, ProcessItem};
use crate::theme::ThemeColors;

static ZOMBIE_IMG: &[u8] = include_bytes!("../../assets/zombie.png");

pub fn render_processes_view<'a>(
    processes: &'a [ProcessItem],
    search_query: &str,
    sort_key: SortKey,
    sort_ascending: bool,
    selected_pid: Option<u32>,
    snapshot_taken: bool,
    diff_mode: bool,
    snapshot_pids: &'a std::collections::HashSet<u32>,
    mem_leaks: &'a std::collections::HashSet<u32>,
    suspended_pids: &'a std::collections::HashSet<u32>,
    boost_active: bool,
    lang: &'a Lang,
) -> Element<'a, Message> {
    let selected_proc = selected_pid.and_then(|pid| processes.iter().find(|p| p.pid == pid));

    // Search bar
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

    // Sort order button
    let sort_label = if sort_ascending { &lang.proc_sort_asc } else { &lang.proc_sort_desc };
    let sort_btn = button(
        text(sort_label)
            .size(12)
            .style(|_| text::Style { color: Some(ThemeColors::CPU_ACCENT) })
    )
    .padding([6, 12])
    .on_press(Message::ToggleSortOrder)
    .style(|_, status| button::Style {
        background: Some(iced::Background::Color(if status == button::Status::Hovered {
            Color::from_rgba(0.02, 0.71, 0.83, 0.15)
        } else {
            Color::from_rgba(0.02, 0.71, 0.83, 0.08)
        })),
        text_color: ThemeColors::CPU_ACCENT,
        border: iced::Border { color: ThemeColors::CPU_ACCENT, width: 1.0, radius: 6.0.into() },
        shadow: Default::default(),
    });

    // Snapshot buttons
    let snapshot_btn = button(
        text(&lang.proc_snapshot_take).size(12)
            .style(|_| text::Style { color: Some(ThemeColors::NET_ACCENT) })
    )
    .padding([6, 12])
    .on_press(Message::TakeSnapshot)
    .style(|_, status| button::Style {
        background: Some(iced::Background::Color(if status == button::Status::Hovered {
            Color::from_rgba(0.15, 0.7, 0.3, 0.2)
        } else {
            Color::from_rgba(0.15, 0.7, 0.3, 0.08)
        })),
        text_color: ThemeColors::NET_ACCENT,
        border: iced::Border { color: ThemeColors::NET_ACCENT, width: 1.0, radius: 6.0.into() },
        shadow: Default::default(),
    });

    let mut snapshot_row = row![sort_btn, Space::with_width(Length::Fixed(8.0)), snapshot_btn];

    if snapshot_taken {
        let diff_btn = button(
            text(if diff_mode { "Exit Diff" } else { &lang.proc_snapshot_diff }).size(12)
                .style(|_| text::Style { color: Some(ThemeColors::DISK_ACCENT) })
        )
        .padding([6, 12])
        .on_press(Message::ToggleDiffMode)
        .style(|_, status| button::Style {
            background: Some(iced::Background::Color(if status == button::Status::Hovered {
                Color::from_rgba(0.9, 0.6, 0.1, 0.2)
            } else {
                Color::from_rgba(0.9, 0.6, 0.1, 0.08)
            })),
            text_color: ThemeColors::DISK_ACCENT,
            border: iced::Border { color: ThemeColors::DISK_ACCENT, width: 1.0, radius: 6.0.into() },
            shadow: Default::default(),
        });
        let clear_btn = button(
            text(&lang.proc_snapshot_clear).size(12)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) })
        )
        .padding([6, 12])
        .on_press(Message::ClearSnapshot)
        .style(|_, status| button::Style {
            background: Some(iced::Background::Color(if status == button::Status::Hovered {
                ThemeColors::CARD_HOVER
            } else {
                ThemeColors::CARD_BG
            })),
            text_color: ThemeColors::TEXT_MUTED,
            border: iced::Border { color: ThemeColors::CARD_BORDER, width: 1.0, radius: 6.0.into() },
            shadow: Default::default(),
        });
        
        let boost_btn = if boost_active {
            button(
                text(&lang.proc_boost_disable).size(12)
                    .style(|_| text::Style { color: Some(Color::WHITE) })
            )
            .padding([6, 12])
            .on_press(Message::ToggleBoostMode)
            .style(|_, status| button::Style {
                background: Some(iced::Background::Color(if status == button::Status::Hovered {
                    Color::from_rgb(0.9, 0.4, 0.1)
                } else {
                    Color::from_rgb(0.8, 0.3, 0.0)
                })),
                text_color: Color::WHITE,
                border: iced::Border { color: Color::from_rgb(0.9, 0.4, 0.1), width: 1.0, radius: 6.0.into() },
                shadow: Default::default(),
            })
        } else {
            button(
                text(&lang.proc_boost_enable).size(12)
                    .style(|_| text::Style { color: Some(Color::WHITE) })
            )
            .padding([6, 12])
            .on_press(Message::ToggleBoostMode)
            .style(|_, status| button::Style {
                background: Some(iced::Background::Color(if status == button::Status::Hovered {
                    Color::from_rgb(0.7, 0.1, 0.9)
                } else {
                    Color::from_rgb(0.6, 0.0, 0.8)
                })),
                text_color: Color::WHITE,
                border: iced::Border { color: Color::from_rgb(0.7, 0.1, 0.9), width: 1.0, radius: 6.0.into() },
                shadow: Default::default(),
            })
        };

        snapshot_row = snapshot_row
            .push(Space::with_width(Length::Fixed(8.0)))
            .push(boost_btn)
            .push(Space::with_width(Length::Fixed(16.0)))
            .push(diff_btn)
            .push(Space::with_width(Length::Fixed(8.0)))
            .push(clear_btn);
    }

    let process_count = text(format!("{}: {}", lang.proc_count, processes.len()))
        .size(13)
        .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) });

    let status_el: Element<Message> = if snapshot_taken {
        text(&lang.proc_snapshot_active).size(12)
            .style(|_| text::Style { color: Some(ThemeColors::NET_ACCENT) })
            .into()
    } else {
        Space::with_width(Length::Fixed(0.0)).into()
    };

    // Actions panel
    let mut actions = row![].spacing(8).align_y(Alignment::Center);

    if let Some(proc) = selected_proc {
        let pid = proc.pid;
        let is_susp = suspended_pids.contains(&pid);

        let suspend_action_btn = if is_susp {
            button(text(format!("{} ({})", lang.proc_resume, pid)).size(12)
                .style(|_| text::Style { color: Some(Color::WHITE) }))
            .padding([8, 14])
            .on_press(Message::ResumeProcess(pid))
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
            button(text(format!("{} ({})", lang.proc_suspend, pid)).size(12)
                .style(|_| text::Style { color: Some(Color::WHITE) }))
            .padding([8, 14])
            .on_press(Message::SuspendProcess(pid))
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
                button(text(format!("{} ({})", lang.proc_kill_tree, pid)).size(12)
                    .style(|_| text::Style { color: Some(Color::WHITE) }))
                .padding([8, 14])
                .on_press(Message::KillProcessTree(pid))
                .style(|_, status| button::Style {
                    background: Some(iced::Background::Color(if status == button::Status::Hovered {
                        Color::from_rgb(0.7, 0.1, 0.3)
                    } else {
                        Color::from_rgb(0.6, 0.0, 0.2)
                    })),
                    text_color: Color::WHITE,
                    border: iced::Border { radius: 6.0.into(), ..Default::default() },
                    shadow: Default::default(),
                })
            )
            .push(suspend_action_btn)
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
    } else {
        actions = actions.push(
            button(text(&lang.proc_select_hint).size(12)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }))
            .padding([8, 14])
            .style(|_, _| button::Style {
                background: Some(iced::Background::Color(Color::TRANSPARENT)),
                text_color: ThemeColors::TEXT_MUTED,
                border: iced::Border {
                    color: ThemeColors::CARD_BORDER,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                shadow: Default::default(),
            })
        );
    }

    let action_bar = row![
        process_count,
        Space::with_width(Length::Fixed(12.0)),
        snapshot_row,
        Space::with_width(Length::Fixed(12.0)),
        status_el,
        Space::with_width(Length::Fill),
        actions,
    ]
    .align_y(Alignment::Center);

    // Header
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
            container(Space::with_width(Length::Fixed(0.0))).width(Length::Fixed(8.0)), // spacer instead of 20px
            container(header_btn(&lang.proc_col_pid, SortKey::Pid, sort_key, sort_ascending)).width(Length::Fixed(72.0)),
            container(header_btn(&lang.proc_col_name, SortKey::Name, sort_key, sort_ascending)).width(Length::Fill),
            container(header_btn(&lang.proc_col_cpu, SortKey::Cpu, sort_key, sort_ascending)).width(Length::Fixed(90.0)),
            container(header_btn(&lang.proc_col_ram, SortKey::Memory, sort_key, sort_ascending)).width(Length::Fixed(110.0)),
            container(header_btn(&lang.proc_col_disk, SortKey::Disk, sort_key, sort_ascending)).width(Length::Fixed(150.0)),
            container(header_btn(&lang.proc_col_age, SortKey::Age, sort_key, sort_ascending)).width(Length::Fixed(90.0)),
            container(text(lang.proc_col_status.clone()).size(12)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }))
                .width(Length::Fixed(90.0)),
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

    // Process rows
    let zombie_handle = image::Handle::from_bytes(ZOMBIE_IMG);
    let mut rows_col = column![].spacing(2);

    for proc in processes {
        let is_selected = selected_pid == Some(proc.pid);
        let pid_val = proc.pid;
        let is_zombie = proc.is_zombie_like();
        let is_leaking = mem_leaks.contains(&proc.pid);

        // Diff mode colors
        let (row_bg_override, diff_tag) = if diff_mode {
            if !snapshot_pids.contains(&proc.pid) {
                (Some(Color::from_rgba(0.1, 0.7, 0.2, 0.12)), Some(lang.proc_diff_new.as_str()))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        let is_suspended = suspended_pids.contains(&proc.pid);
        let mut alert_parts = Vec::new();
        let mut text_color = ThemeColors::TEXT_PRIMARY;

        if is_suspended { alert_parts.push(lang.proc_suspended_badge.as_str()); if text_color == ThemeColors::TEXT_PRIMARY { text_color = Color::from_rgb(0.9, 0.6, 0.1); } }
        if proc.cpu_usage > 50.0 { alert_parts.push(" [!]CPU"); text_color = ThemeColors::DANGER; }
        if proc.memory_bytes > 1_073_741_824 { alert_parts.push(" [!]RAM"); if text_color == ThemeColors::TEXT_PRIMARY { text_color = ThemeColors::RAM_ACCENT; } }
        if is_leaking { alert_parts.push(" [LEAK]"); if text_color == ThemeColors::TEXT_PRIMARY { text_color = Color::from_rgb(1.0, 0.6, 0.0); } }
        if let Some(tag) = diff_tag { alert_parts.push(tag); }

        let name_label = format!("{}{}", proc.name, alert_parts.join(""));

        let cpu_color = if proc.cpu_usage > 50.0 { ThemeColors::DANGER }
            else if proc.cpu_usage > 20.0 { ThemeColors::RAM_ACCENT }
            else { ThemeColors::CPU_ACCENT };

        let age_str = if proc.age_secs() == 0 { "-".to_string() }
            else { format_duration(proc.age_secs()) };

        let base_bg = row_bg_override.unwrap_or(ThemeColors::CARD_BG);

        let mut row_content = row![].align_y(Alignment::Center).padding([6, 12]);

        if is_zombie {
            row_content = row_content
                .push(image(zombie_handle.clone()).width(Length::Fixed(18.0)).height(Length::Fixed(18.0)))
                .push(Space::with_width(Length::Fixed(4.0)));
        }

        let mut sparkline_row = row![].spacing(1).height(Length::Fixed(12.0)).align_y(Alignment::End);
        for &cpu_val in &proc.cpu_history {
            let h_pct = (cpu_val / 100.0).clamp(0.05, 1.0) as f32;
            let h_pixels = h_pct * 12.0;
            let bar_color = if cpu_val > 50.0 { ThemeColors::DANGER }
                else if cpu_val > 20.0 { ThemeColors::RAM_ACCENT }
                else { ThemeColors::CPU_ACCENT };
            
            sparkline_row = sparkline_row.push(
                container(Space::with_width(Length::Fixed(0.0)))
                    .width(Length::Fixed(2.0))
                    .height(Length::Fixed(h_pixels))
                    .style(move |_| container::Style {
                        background: Some(iced::Background::Color(bar_color)),
                        ..Default::default()
                    })
            );
        }
        
        let cpu_col = column![
            text(format!("{:.1}%", proc.cpu_usage)).size(12).style(move |_| text::Style { color: Some(cpu_color) }),
            sparkline_row
        ].width(Length::Fixed(90.0)).spacing(2);

        row_content = row_content
            .push(
                text(proc.pid.to_string())
                    .size(12)
                    .width(Length::Fixed(72.0))
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            )
            .push(
                text(name_label)
                    .size(12)
                    .width(Length::Fill)
                    .style(move |_| text::Style { color: Some(text_color) }),
            )
            .push(cpu_col)
            .push(
                text(format_bytes(proc.memory_bytes))
                    .size(12)
                    .width(Length::Fixed(110.0))
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            )
            .push(
                text(format!("{} / {}", format_rate(proc.disk_read_bytes_sec), format_rate(proc.disk_write_bytes_sec)))
                    .size(11)
                    .width(Length::Fixed(150.0))
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            )
            .push(
                text(age_str)
                    .size(11)
                    .width(Length::Fixed(90.0))
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            )
            .push(
                text(&proc.status)
                    .size(11)
                    .width(Length::Fixed(90.0))
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            );

        let row_btn = button(row_content)
            .width(Length::Fill)
            .on_press(Message::SelectProcess(if is_selected { None } else { Some(pid_val) }))
            .style(move |_, status| {
                let bg = if is_selected {
                    Color::TRANSPARENT
                } else if status == button::Status::Hovered {
                    ThemeColors::CARD_HOVER
                } else {
                    base_bg
                };
                button::Style {
                    background: Some(iced::Background::Color(bg)),
                    text_color: ThemeColors::TEXT_PRIMARY,
                    border: iced::Border {
                        color: if is_selected { ThemeColors::CPU_ACCENT } else { Color::TRANSPARENT },
                        width: if is_selected { 1.0 } else { 0.0 },
                        radius: 4.0.into(),
                    },
                    shadow: Default::default(),
                }
            });

        rows_col = rows_col.push(row_btn);
    }

    let scrollable_table = scrollable(rows_col).height(Length::Fill);

    let content = column![
        text(&lang.proc_title).size(20)
            .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
        Space::with_height(Length::Fixed(16.0)),
        search_input,
        Space::with_height(Length::Fixed(12.0)),
        action_bar,
        Space::with_height(Length::Fixed(12.0)),
        header_row,
        Space::with_height(Length::Fixed(6.0)),
        scrollable_table,
    ]
    .padding(24);

    container(content).into()
}

