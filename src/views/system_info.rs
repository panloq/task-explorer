use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Alignment, Element, Length};
use crate::app::Message;
use crate::system::{format_bytes, format_duration, CpuInfo, MemoryInfo, SystemOverview};
use crate::theme::ThemeColors;
use crate::lang::Lang;

pub fn render_system_info_view<'a>(
    sys: &SystemOverview,
    cpu: &CpuInfo,
    mem: &MemoryInfo,
    lang: &'a Lang,
) -> Element<'a, Message> {
    fn render_info_item<'a>(icon: &'a str, label: &'a str, val: String) -> Element<'a, Message> {
        container(
            row![
                text(icon).size(22),
                Space::with_width(Length::Fixed(12.0)),
                column![
                    text(label)
                        .size(11)
                        .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                    Space::with_height(Length::Fixed(2.0)),
                    text(val)
                        .size(14)
                        .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
                ]
            ]
            .align_y(Alignment::Center)
        )
        .padding(14)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(ThemeColors::CARD_HOVER)),
            border: iced::Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    }

    let item1 = render_info_item("OS", &lang.sysinfo_os, format!("{} ({})", sys.os_name, sys.os_version));
    let item2 = render_info_item("HOST", &lang.sysinfo_hostname, sys.host_name.clone());
    let item3 = render_info_item("KERN", &lang.sysinfo_kernel, sys.kernel_version.clone());
    let item4 = render_info_item("TIME", &lang.sysinfo_uptime, format_duration(sys.uptime_secs));

    let row1 = row![item1, Space::with_width(Length::Fixed(12.0)), item2].width(Length::Fill);
    let row2 = row![item3, Space::with_width(Length::Fixed(12.0)), item4].width(Length::Fill);

    let sys_card = container(
        column![
            text(&lang.sysinfo_sys_id)
                .size(16)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            Space::with_height(Length::Fixed(12.0)),
            row1,
            Space::with_height(Length::Fixed(12.0)),
            row2,
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

    let cpu_item1 = render_info_item("CPU", &lang.sysinfo_cpu_model, cpu.brand.clone());
    let cpu_item2 = render_info_item("GPU", &lang.sysinfo_gpu_model, sys.gpu_name.clone());
    let cpu_item3 = render_info_item("CORE", &lang.sysinfo_cpu_cores, format!("{} {} / {} {}", cpu.physical_core_count, lang.sysinfo_phys, cpu.logical_core_count, lang.sysinfo_logical));
    let cpu_item4 = render_info_item("FREQ", &lang.sysinfo_base_freq, format!("{:.2} GHz", cpu.frequency_mhz as f64 / 1000.0));
    let cpu_item5 = render_info_item("RAM", &lang.sysinfo_ram_installed, format_bytes(mem.total_bytes));

    let cpu_row1 = row![cpu_item1, Space::with_width(Length::Fixed(12.0)), cpu_item2].width(Length::Fill);
    let cpu_row2 = row![cpu_item3, Space::with_width(Length::Fixed(12.0)), cpu_item4].width(Length::Fill);
    let cpu_row3 = row![cpu_item5].width(Length::Fill);

    let hw_card = container(
        column![
            text(&lang.sysinfo_hw_spec)
                .size(16)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            Space::with_height(Length::Fixed(12.0)),
            cpu_row1,
            Space::with_height(Length::Fixed(12.0)),
            cpu_row2,
            Space::with_height(Length::Fixed(12.0)),
            cpu_row3,
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

    let content = column![
        text(&lang.sysinfo_page_title)
            .size(20)
            .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
        Space::with_height(Length::Fixed(16.0)),
        sys_card,
        Space::with_height(Length::Fixed(20.0)),
        hw_card
    ]
    .padding(24);

    scrollable(content).into()
}
