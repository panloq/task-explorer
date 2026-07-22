use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Alignment, Element, Length};
use crate::app::Message;
use crate::components::progress_bar::custom_progress_bar;
use crate::system::{format_bytes, format_rate, DiskInfo, ProcessItem};
use crate::theme::ThemeColors;
use crate::lang::Lang;

pub fn render_disks_view<'a>(
    disks: &[DiskInfo],
    processes: &'a [ProcessItem],
    lang: &'a Lang,
) -> Element<'a, Message> {
    // 1. Calculate overall IO rate
    let total_read_sec: u64 = processes.iter().map(|p| p.disk_read_bytes_sec).sum();
    let total_write_sec: u64 = processes.iter().map(|p| p.disk_write_bytes_sec).sum();

    // 2. Build Disk List (Left side)
    let mut disk_col = column![].spacing(16);
    for disk in disks {
        let title = if disk.name.is_empty() {
            format!("{} ({})", lang.disk_label, disk.mount_point)
        } else {
            format!("{} ({})", disk.name, disk.mount_point)
        };

        let used = disk.used_bytes();
        let percent = disk.used_percent();

        let header = row![
            text(&lang.disk_header)
                .size(16)
                .style(|_| text::Style { color: Some(ThemeColors::DISK_ACCENT) }),
            Space::with_width(Length::Fixed(12.0)),
            column![
                text(title)
                    .size(15)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
                text(format!("{}: {} | {}: {} ({})", lang.disk_fs, disk.fs_type, lang.disk_type, if disk.is_removable { &lang.disk_removable } else { &lang.disk_fixed }, disk.disk_kind))
                    .size(11)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            ]
        ]
        .align_y(Alignment::Center);

        let space_info = row![
            text(format!("{}: {}", lang.disk_used, format_bytes(used)))
                .size(12)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            Space::with_width(Length::Fill),
            text(format!("{}: {} / {}", lang.disk_free, format_bytes(disk.available_bytes), format_bytes(disk.total_bytes)))
                .size(12)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
        ];

        let card = container(
            column![
                header,
                Space::with_height(Length::Fixed(12.0)),
                space_info,
                Space::with_height(Length::Fixed(8.0)),
                custom_progress_bar(percent, ThemeColors::DISK_ACCENT),
                Space::with_height(Length::Fixed(4.0)),
                text(format!("{}: {:.1}%", lang.disk_fill, percent))
                    .size(11)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) })
            ]
        )
        .padding(16)
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

        disk_col = disk_col.push(card);
    }

    // 3. Right side widgets: Realtime IO Rate Card
    let io_summary_card = container(
        column![
            text(&lang.disk_io_activity)
                .size(13)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            Space::with_height(Length::Fixed(10.0)),
            row![
                column![
                    text(lang.disk_read_rate.to_uppercase())
                        .size(10)
                        .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                    text(format!("↓ {}", format_rate(total_read_sec)))
                        .size(18)
                        .style(|_| text::Style { color: Some(ThemeColors::DISK_ACCENT) }),
                ].width(Length::Fill),
                column![
                    text(lang.disk_write_rate.to_uppercase())
                        .size(10)
                        .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                    text(format!("↑ {}", format_rate(total_write_sec)))
                        .size(18)
                        .style(|_| text::Style { color: Some(ThemeColors::CPU_ACCENT) }),
                ].width(Length::Fill)
            ]
        ]
    )
    .padding(16)
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

    // Top 3 Active IO processes
    let mut active_proc_list = processes.to_vec();
    active_proc_list.sort_by_key(|p| std::cmp::Reverse(p.disk_read_bytes_sec + p.disk_write_bytes_sec));
    
    let mut active_proc_rows = column![].spacing(6);
    for p in active_proc_list.iter().filter(|p| (p.disk_read_bytes_sec + p.disk_write_bytes_sec) > 0).take(3) {
        let speed = p.disk_read_bytes_sec + p.disk_write_bytes_sec;
        let row_item = row![
            text(format!("{} (PID: {})", p.name, p.pid))
                .size(12)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) })
                .width(Length::Fill),
            text(format_rate(speed))
                .size(12)
                .style(|_| text::Style { color: Some(ThemeColors::DISK_ACCENT) })
        ];
        active_proc_rows = active_proc_rows.push(row_item);
    }
    if active_proc_list.iter().filter(|p| (p.disk_read_bytes_sec + p.disk_write_bytes_sec) > 0).count() == 0 {
        active_proc_rows = active_proc_rows.push(
            text(&lang.disk_no_io)
                .size(11)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) })
        );
    }

    let top_active_card = container(
        column![
            text(&lang.disk_active_procs)
                .size(13)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            Space::with_height(Length::Fixed(8.0)),
            active_proc_rows
        ]
    )
    .padding(16)
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

    // SSD Wear Guard (Lifetime Writes Tracker)
    let mut wear_proc_list = processes.to_vec();
    wear_proc_list.sort_by_key(|p| std::cmp::Reverse(p.total_written_bytes));

    let mut wear_proc_rows = column![].spacing(6);
    for p in wear_proc_list.iter().filter(|p| p.total_written_bytes > 1_000_000).take(3) {
        let is_excessive = p.total_written_bytes > 5_000_000_000; // 5 GB
        let color = if is_excessive { ThemeColors::DANGER } else { ThemeColors::TEXT_PRIMARY };
        let warning_tag = if is_excessive { format!(" [{}]", lang.disk_high_write) } else { "".to_string() };

        let row_item = row![
            text(format!("{}{}", p.name, warning_tag))
                .size(12)
                .style(move |_| text::Style { color: Some(color) })
                .width(Length::Fill),
            text(format!("R: {} | W: {}", format_bytes(p.total_read_bytes), format_bytes(p.total_written_bytes)))
                .size(11)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) })
        ];
        wear_proc_rows = wear_proc_rows.push(row_item);
    }
    if wear_proc_list.iter().filter(|p| p.total_written_bytes > 1_000_000).count() == 0 {
        wear_proc_rows = wear_proc_rows.push(
            text(&lang.disk_no_wear_data)
                .size(11)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) })
        );
    }

    let ssd_wear_card = container(
        column![
            text(format!("SSD {} ({})", lang.disk_wear_guard, if lang.disk_wear_guard == "Straż zużycia" { "Zapis od startu procesu" } else { "Writes since process start" }))
                .size(13)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            Space::with_height(Length::Fixed(8.0)),
            wear_proc_rows
        ]
    )
    .padding(16)
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

    let right_col = column![
        io_summary_card,
        Space::with_height(Length::Fixed(16.0)),
        top_active_card,
        Space::with_height(Length::Fixed(16.0)),
        ssd_wear_card
    ]
    .width(Length::FillPortion(2));

    let dashboard = row![
        disk_col.width(Length::FillPortion(3)),
        Space::with_width(Length::Fixed(16.0)),
        right_col
    ]
    .width(Length::Fill);

    let content = column![
        text(&lang.disk_page_title)
            .size(20)
            .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
        Space::with_height(Length::Fixed(16.0)),
        dashboard
    ]
    .padding(24);

    scrollable(content).into()
}
