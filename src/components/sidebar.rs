use iced::widget::{button, column, container, row, text, image, Space};
use iced::{Alignment, Color, Element, Length};
use crate::app::{Message, ProcessesSubTab, Tab};
use crate::lang::Lang;
use crate::theme::ThemeColors;

static APP_ICON_BYTES: &[u8] = include_bytes!("../../assets/app_icon.png");

pub fn render_sidebar<'a>(
    current_tab: Tab,
    processes_subtab: ProcessesSubTab,
    processes_expanded: bool,
    process_count: usize,
    lang: &'a Lang,
    refresh_rate: f32,
) -> Element<'a, Message> {
    let logo_img = image(iced::widget::image::Handle::from_bytes(APP_ICON_BYTES))
        .width(Length::Fixed(28.0))
        .height(Length::Fixed(28.0));

    let logo = row![
        logo_img,
        text("TaskExplorer")
            .size(18)
            .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let logo_container = container(logo).padding(16).width(Length::Fill);

    let tabs: Vec<(Tab, String)> = vec![
        (Tab::Overview, lang.tab_overview.to_string()),
        (Tab::Cpu, "CPU".to_string()),
        (Tab::Ram, "RAM".to_string()),
        (Tab::Processes, if process_count > 0 {
            format!("{} ({})", lang.tab_processes, process_count)
        } else {
            lang.tab_processes.to_string()
        }),
        (Tab::Disks, lang.tab_disks.to_string()),
        (Tab::Network, lang.tab_network.to_string()),
        (Tab::SystemInfo, lang.tab_system_info.to_string()),
        (Tab::Settings, lang.tab_settings.to_string()),
    ];

    let mut nav_col = column![].spacing(6).padding(10);

    for (tab, label) in tabs {
        let is_selected = current_tab == tab;

        let arrow = if tab == Tab::Processes {
            if processes_expanded { "  v" } else { "  >" }
        } else {
            ""
        };

        let btn_label = format!("{}{}", label, arrow);

        let btn_content = row![
            text(btn_label)
                .size(14)
                .style(move |_| text::Style {
                    color: Some(if is_selected { ThemeColors::TEXT_PRIMARY } else { ThemeColors::TEXT_MUTED }),
                })
        ]
        .align_y(Alignment::Center)
        .padding([10, 14]);

        let nav_btn = button(btn_content)
            .width(Length::Fill)
            .on_press(if tab == Tab::Processes {
                Message::ToggleProcessesExpanded
            } else {
                Message::TabSelected(tab)
            })
            .style(move |_, status| button::Style {
                background: Some(iced::Background::Color(if is_selected {
                    Color::from_rgba(0.02, 0.71, 0.83, 0.2)
                } else if status == button::Status::Hovered {
                    ThemeColors::CARD_HOVER
                } else {
                    Color::TRANSPARENT
                })),
                text_color: if is_selected { ThemeColors::TEXT_PRIMARY } else { ThemeColors::TEXT_MUTED },
                border: iced::Border {
                    color: if is_selected { ThemeColors::CPU_ACCENT } else { Color::TRANSPARENT },
                    width: if is_selected { 1.0 } else { 0.0 },
                    radius: 6.0.into(),
                },
                shadow: Default::default(),
            });

        nav_col = nav_col.push(nav_btn);

        // Accordion sub-menu under Processes
        if tab == Tab::Processes && processes_expanded {
            let sub_items = vec![
                (ProcessesSubTab::Processes, format!("  • {}", lang.proc_subtab_processes)),
                (ProcessesSubTab::Startup, format!("  • {}", lang.proc_subtab_autostart)),
                (ProcessesSubTab::Details, format!("  • {}", lang.proc_subtab_details)),
            ];

            let mut sub_col = column![].spacing(3).padding([2, 10]);

            for (subtab, sub_label) in sub_items {
                let is_sub_selected = current_tab == Tab::Processes && processes_subtab == subtab;

                let sub_btn_content = row![
                    text(sub_label)
                        .size(12)
                        .style(move |_| text::Style {
                            color: Some(if is_sub_selected { ThemeColors::CPU_ACCENT } else { ThemeColors::TEXT_MUTED }),
                        })
                ]
                .align_y(Alignment::Center)
                .padding([6, 10]);

                let sub_btn = button(sub_btn_content)
                    .width(Length::Fill)
                    .on_press(Message::ProcessesSubTabSelected(subtab))
                    .style(move |_, status| button::Style {
                        background: Some(iced::Background::Color(if is_sub_selected {
                            Color::from_rgba(0.02, 0.71, 0.83, 0.12)
                        } else if status == button::Status::Hovered {
                            ThemeColors::CARD_HOVER
                        } else {
                            Color::TRANSPARENT
                        })),
                        text_color: if is_sub_selected { ThemeColors::CPU_ACCENT } else { ThemeColors::TEXT_MUTED },
                        border: iced::Border {
                            color: if is_sub_selected { ThemeColors::CPU_ACCENT } else { Color::TRANSPARENT },
                            width: if is_sub_selected { 1.0 } else { 0.0 },
                            radius: 4.0.into(),
                        },
                        shadow: Default::default(),
                    });

                sub_col = sub_col.push(sub_btn);
            }

            nav_col = nav_col.push(sub_col);
        }
    }

    let footer = container(
        column![
            text(format!("{} {:.1}s", lang.auto_refresh_label, refresh_rate))
                .size(11)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            text("Iced v0.13 + Sysinfo")

                .size(10)
                .style(|_| text::Style { color: Some(Color::from_rgb(0.4, 0.45, 0.52)) }),
        ]
        .spacing(4)
    )
    .padding(16);

    let content = column![
        logo_container,
        Space::with_height(Length::Fixed(10.0)),
        nav_col,
        Space::with_height(Length::Fill),
        footer,
    ];

    container(content)
        .width(Length::Fixed(220.0))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(ThemeColors::SIDEBAR_BG)),
            border: iced::Border {
                color: ThemeColors::CARD_BORDER,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}
