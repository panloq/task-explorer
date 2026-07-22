use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Alignment, Color, Element, Length};
use std::collections::HashSet;
use crate::app::Message;
use crate::components::chart::{cpu_chart_widget, ram_chart_widget};
use crate::components::progress_bar::custom_progress_bar;
use crate::components::stat_card::render_stat_card;
use crate::lang::Lang;
use crate::system::{format_bytes, format_rate, CpuInfo, DiskInfo, MemoryInfo, NetworkInterfaceInfo, ProcessItem};
use crate::theme::ThemeColors;

pub fn render_overview_view<'a>(
    cpu: &CpuInfo,
    mem: &MemoryInfo,
    disks: &[DiskInfo],
    networks: &[NetworkInterfaceInfo],
    cpu_history: &'a [f32],
    memory_history: &'a [f32],
    lang: &'a Lang,
    leak_pids: &'a HashSet<u32>,
    processes: &'a [ProcessItem],
) -> Element<'a, Message> {
    // --- Top stat cards ---
    let cpu_card = render_stat_card(
        &lang.overview_cpu,
        format!("{:.1}%", cpu.global_usage),
        format!("{} {} | {:.1} GHz", cpu.logical_core_count, lang.overview_cores, cpu.frequency_mhz as f64 / 1000.0),
        Some(cpu.global_usage),
        ThemeColors::CPU_ACCENT,
        crate::app::Tab::Cpu,
    );
    let mem_card = render_stat_card(
        &lang.overview_ram,
        format!("{} / {}", format_bytes(mem.used_bytes), format_bytes(mem.total_bytes)),
        format!("Swap: {} / {}", format_bytes(mem.swap_used_bytes), format_bytes(mem.swap_total_bytes)),
        Some(mem.used_percent()),
        ThemeColors::RAM_ACCENT,
        crate::app::Tab::Ram,
    );
    let total_disk_bytes: u64 = disks.iter().map(|d| d.total_bytes).sum();
    let total_used_bytes: u64 = disks.iter().map(|d| d.used_bytes()).sum();
    let disk_percent = if total_disk_bytes > 0 {
        (total_used_bytes as f64 / total_disk_bytes as f64 * 100.0) as f32
    } else { 0.0 };
    let disk_card = render_stat_card(
        &lang.overview_disk,
        format!("{} / {}", format_bytes(total_used_bytes), format_bytes(total_disk_bytes)),
        format!("{} {}", disks.len(), lang.overview_partitions),
        Some(disk_percent),
        ThemeColors::DISK_ACCENT,
        crate::app::Tab::Disks,
    );
    let total_rx: u64 = networks.iter().map(|n| n.rx_bytes_sec).sum();
    let total_tx: u64 = networks.iter().map(|n| n.tx_bytes_sec).sum();
    let net_card = render_stat_card(
        &lang.overview_network,
        format!("IN: {}  OUT: {}", format_rate(total_rx), format_rate(total_tx)),
        format!("{} {}", networks.len(), lang.overview_interfaces),
        None,
        ThemeColors::NET_ACCENT,
        crate::app::Tab::Network,
    );

    let top_cards = row![
        cpu_card, Space::with_width(Length::Fixed(12.0)),
        mem_card, Space::with_width(Length::Fixed(12.0)),
        disk_card, Space::with_width(Length::Fixed(12.0)),
        net_card,
    ].width(Length::Fill);

    // --- RAM Forecast ---
    let forecast_el: Element<Message> = {
        // Compute linear trend slope (% per tick)
        let n = memory_history.len() as f64;
        let slope = if n >= 10.0 {
            let x_mean = (n - 1.0) / 2.0;
            let y_mean: f64 = memory_history.iter().map(|&v| v as f64).sum::<f64>() / n;
            let (mut num, mut den) = (0.0_f64, 0.0_f64);
            for (i, &y) in memory_history.iter().enumerate() {
                let x = i as f64 - x_mean;
                num += x * (y as f64 - y_mean);
                den += x * x;
            }
            if den == 0.0 { 0.0 } else { num / den }
        } else { 0.0 };

        let current_pct = mem.used_percent() as f64;
        let forecast_text = if slope > 0.05 {
            let ticks_to_full = (100.0 - current_pct) / slope;
            let secs = ticks_to_full as u64;
            if secs < 3600 {
                format!("{}: ~{} {}", lang.forecast_full_in, secs / 60 + 1, lang.forecast_minutes)
            } else {
                format!("{}: ~{:.1} {}", lang.forecast_full_in, secs as f64 / 3600.0, lang.forecast_hours)
            }
        } else {
            lang.forecast_stable.to_string()
        };

        let forecast_color = if slope > 0.05 { ThemeColors::DANGER } else { ThemeColors::NET_ACCENT };

        container(
            row![
                text(&lang.forecast_title).size(13)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                Space::with_width(Length::Fixed(12.0)),
                text(forecast_text).size(13)
                    .style(move |_| text::Style { color: Some(forecast_color) }),
            ]
            .align_y(Alignment::Center)
        )
        .padding([10, 16])
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
            border: iced::Border { color: ThemeColors::CARD_BORDER, width: 1.0, radius: 8.0.into() },
            ..Default::default()
        })
        .into()
    };

    // --- Memory Leaks Summary ---
    let leak_section: Element<Message> = if !leak_pids.is_empty() {
        let mut leak_col = column![
            text(&lang.leak_title).size(13)
                .style(|_| text::Style { color: Some(Color::from_rgb(1.0, 0.6, 0.0)) }),
            Space::with_height(Length::Fixed(6.0)),
        ].spacing(4);

        let leaking: Vec<&ProcessItem> = processes.iter()
            .filter(|p| leak_pids.contains(&p.pid))
            .take(5)
            .collect();

        for proc in leaking {
            leak_col = leak_col.push(
                text(format!("  {} (PID {}) - {} {}", proc.name, proc.pid, format_bytes(proc.memory_bytes), lang.leak_growing))
                    .size(12)
                    .style(|_| text::Style { color: Some(Color::from_rgb(1.0, 0.6, 0.0)) })
            );
        }

        container(leak_col)
            .padding([12, 16])
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(Color::from_rgba(1.0, 0.5, 0.0, 0.08))),
                border: iced::Border { color: Color::from_rgb(1.0, 0.6, 0.0), width: 1.0, radius: 8.0.into() },
                ..Default::default()
            })
            .into()
    } else {
        Space::with_height(Length::Fixed(0.0)).into()
    };

    // --- Charts ---
    let cpu_chart_section = container(
        column![
            row![
                text(&lang.overview_cpu_chart).size(13)
                    .style(|_| text::Style { color: Some(ThemeColors::CPU_ACCENT) }),
                Space::with_width(Length::Fill),
                text(format!("{}: {:.1}%", lang.overview_now, cpu.global_usage)).size(13)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            ].align_y(Alignment::Center),
            Space::with_height(Length::Fixed(8.0)),
            cpu_chart_widget(cpu_history),
        ]
    )
    .padding(16).width(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
        border: iced::Border { color: ThemeColors::CPU_ACCENT, width: 1.0, radius: 8.0.into() },
        ..Default::default()
    });

    let ram_chart_section = container(
        column![
            row![
                text(&lang.overview_ram_chart).size(13)
                    .style(|_| text::Style { color: Some(ThemeColors::RAM_ACCENT) }),
                Space::with_width(Length::Fill),
                text(format!("{}: {:.1}%", lang.overview_now, mem.used_percent())).size(13)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            ].align_y(Alignment::Center),
            Space::with_height(Length::Fixed(8.0)),
            ram_chart_widget(memory_history),
        ]
    )
    .padding(16).width(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
        border: iced::Border { color: ThemeColors::RAM_ACCENT, width: 1.0, radius: 8.0.into() },
        ..Default::default()
    });

    let charts_row = row![
        cpu_chart_section, Space::with_width(Length::Fixed(16.0)), ram_chart_section,
    ].width(Length::Fill);

    // --- CPU Core grid ---
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

    let cpu_section = container(
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
    .padding(16).width(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
        border: iced::Border { color: ThemeColors::CARD_BORDER, width: 1.0, radius: 8.0.into() },
        ..Default::default()
    });

    let main_content = column![
        text(&lang.overview_title).size(20)
            .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
        Space::with_height(Length::Fixed(16.0)),
        top_cards,
        Space::with_height(Length::Fixed(12.0)),
        forecast_el,
        Space::with_height(Length::Fixed(8.0)),
        leak_section,
        Space::with_height(Length::Fixed(12.0)),
        charts_row,
        Space::with_height(Length::Fixed(20.0)),
        cpu_section,
    ]
    .padding(24);

    scrollable(main_content).into()
}
