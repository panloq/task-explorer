use iced::widget::progress_bar;
use iced::{Color, Element, Length};
use crate::app::Message;

pub fn custom_progress_bar<'a>(percent: f32, color: Color) -> Element<'a, Message> {
    let clamped = percent.clamp(0.0, 100.0);
    progress_bar(0.0..=100.0, clamped)
        .width(Length::Fill)
        .height(Length::Fixed(8.0))
        .style(move |_theme: &iced::Theme| {
            progress_bar::Style {
                background: iced::Background::Color(Color::from_rgb(0.20, 0.25, 0.33)),
                bar: iced::Background::Color(color),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
            }
        })
        .into()
}
