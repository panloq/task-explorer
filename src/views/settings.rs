use iced::widget::{button, column, container, row, text, image, Space, slider};
use iced::{Alignment, Color, Element, Length};
use crate::app::Message;
use crate::config::Language;
use crate::lang::Lang;
use crate::theme::ThemeColors;

// Embed flag images at compile time so they always work regardless of working directory
static EN_FLAG_BYTES: &[u8] = include_bytes!("../../assets/en_flag.png");
static PL_FLAG_BYTES: &[u8] = include_bytes!("../../assets/pl_flag.png");
static RU_FLAG_BYTES: &[u8] = include_bytes!("../../assets/ru_flag.png");

pub fn render_settings_view<'a>(lang: &'a Lang, current_language: Language, refresh_rate: f32) -> Element<'a, Message> {
    fn lang_btn<'a>(
        label: &'a str,
        lang_variant: Language,
        current: Language,
        flag_bytes: &'static [u8],
    ) -> Element<'a, Message> {
        let is_active = current == lang_variant;
        let handle = iced::widget::image::Handle::from_bytes(flag_bytes);
        let flag_img = image(handle)
            .width(Length::Fixed(32.0))
            .height(Length::Fixed(22.0))
            .content_fit(iced::ContentFit::Cover);

        let btn_content = row![
            flag_img,
            text(label)
                .size(14)
                .style(move |_| text::Style {
                    color: Some(if is_active { Color::WHITE } else { ThemeColors::TEXT_MUTED }),
                })
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        button(btn_content)
        .padding([12, 28])
        .on_press(Message::SetLanguage(lang_variant))
        .style(move |_, status| button::Style {
            background: Some(iced::Background::Color(if is_active {
                ThemeColors::CPU_ACCENT
            } else if status == button::Status::Hovered {
                ThemeColors::CARD_HOVER
            } else {
                ThemeColors::CARD_BG
            })),
            text_color: if is_active { Color::WHITE } else { ThemeColors::TEXT_MUTED },
            border: iced::Border {
                color: if is_active { ThemeColors::CPU_ACCENT } else { ThemeColors::CARD_BORDER },
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Default::default(),
        })
        .into()
    }

    let lang_section = container(
        column![
            text(lang.settings_language.clone())
                .size(15)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
            Space::with_height(Length::Fixed(16.0)),
            row![
                lang_btn(&lang.settings_lang_en, Language::English, current_language, EN_FLAG_BYTES),
                Space::with_width(Length::Fixed(12.0)),
                lang_btn(&lang.settings_lang_pl, Language::Polish, current_language, PL_FLAG_BYTES),
                Space::with_width(Length::Fixed(12.0)),
                lang_btn(&lang.settings_lang_ru, Language::Russian, current_language, RU_FLAG_BYTES),
            ]
            .align_y(Alignment::Center),
        ]
    )
    .padding(20)
    .width(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
        border: iced::Border {
            color: ThemeColors::CARD_BORDER,
            width: 1.0,
            radius: 10.0.into(),
        },
        ..Default::default()
    });

    let content = column![
        text(lang.settings_title.clone())
            .size(22)
            .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
        Space::with_height(Length::Fixed(24.0)),
        lang_section,
        Space::with_height(Length::Fixed(24.0)),

        container(
            column![
                text(&lang.settings_refresh_rate)
                    .size(15)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) }),
                Space::with_height(Length::Fixed(16.0)),
                row![
                    text(format!("{:.1}s", refresh_rate))
                        .size(14)
                        .width(Length::Fixed(40.0))
                        .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                    slider(1.0..=5.0, refresh_rate, Message::SetRefreshRate)
                        .step(0.1_f32)
                        .width(Length::Fixed(200.0)),
                ]
                .align_y(Alignment::Center),
            ]
        )
        .padding(20)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
            border: iced::Border {
                color: ThemeColors::CARD_BORDER,
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        }),
        Space::with_height(Length::Fixed(24.0)),

        // Info card
        container(
            column![
                text("TaskExplorer")
                    .size(16)
                    .style(|_| text::Style { color: Some(ThemeColors::CPU_ACCENT) }),
                Space::with_height(Length::Fixed(6.0)),
                text("v0.1.0 - Built with Rust + Iced")
                    .size(12)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
                text("System monitoring with plotters-iced charts")
                    .size(12)
                    .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) }),
            ]
        )
        .padding(20)
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
            border: iced::Border {
                color: ThemeColors::CARD_BORDER,
                width: 1.0,
                radius: 10.0.into(),
            },
            ..Default::default()
        }),
    ]
    .padding(32);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
