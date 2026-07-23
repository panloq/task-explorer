use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Color, Element, Length};
use crate::app::Message;
use crate::lang::Lang;
use crate::system::ProcessItem;
use crate::theme::ThemeColors;

pub fn render_anomalies_view<'a>(
    processes: &'a [ProcessItem],
    _lang: &'a Lang,
) -> Element<'a, Message> {
    let mut anomalies = Vec::new();

    // Skanowanie (Heurystyki)
    for proc in processes {
        let path_lower = proc.exe_path.to_lowercase();
        let name_lower = proc.name.to_lowercase();
        
        let mut reasons = Vec::new();

        // 1. Działa z Temp
        if path_lower.contains("\\appdata\\local\\temp\\") {
            reasons.push("Running from Temp folder");
        }
        
        // 2. Headless + Duże zużycie CPU
        // Nie mamy łatwo jak sprawdzić headless, ale wysokie CPU bez znanego powiązania:
        if proc.cpu_usage > 40.0 && proc.parent_pid.is_none() && !name_lower.ends_with("exe") && proc.exe_path.is_empty() {
            reasons.push("High CPU, no parent, unknown path");
        }
        
        // 3. Typowe nazwy krypto-koparek lub malware (bardzo podstawowe)
        if name_lower.contains("miner") || name_lower.contains("xmrig") {
            reasons.push("Suspicious name (miner)");
        }

        if !reasons.is_empty() {
            anomalies.push((proc, reasons));
        }
    }

    if anomalies.is_empty() {
        return container(
            text("No anomalies detected. System seems clean.")
                .size(16)
                .style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) })
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into();
    }

    let mut rows_col = column![].spacing(8);

    for (proc, reasons) in anomalies {
        let name_label = text(&proc.name).size(14).style(|_| text::Style { color: Some(ThemeColors::TEXT_PRIMARY) });
        let pid_label = text(format!("PID: {}", proc.pid)).size(12).style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) });
        let path_label = text(&proc.exe_path).size(12).style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) });
        
        let reasons_col = column(
            reasons.iter().map(|r| text(format!("• {}", r)).size(12).style(|_| text::Style { color: Some(ThemeColors::DANGER) }).into()).collect::<Vec<_>>()
        ).spacing(2);

        let kill_btn = button(text("Kill").size(12))
            .padding([4, 8])
            .on_press(Message::KillProcess(proc.pid))
            .style(|_, status| button::Style {
                background: Some(iced::Background::Color(if status == button::Status::Hovered {
                    Color::from_rgb(0.8, 0.2, 0.2)
                } else {
                    ThemeColors::DANGER
                })),
                text_color: Color::WHITE,
                border: iced::Border { radius: 4.0.into(), ..Default::default() },
                shadow: Default::default(),
            });

        let row_content = row![
            column![
                row![name_label, Space::with_width(Length::Fixed(8.0)), pid_label].align_y(Alignment::Center),
                path_label,
                Space::with_height(Length::Fixed(4.0)),
                reasons_col,
            ].width(Length::Fill),
            kill_btn
        ].align_y(Alignment::Center).padding(12);

        let card = container(row_content)
            .style(|_| container::Style {
                background: Some(iced::Background::Color(ThemeColors::CARD_BG)),
                border: iced::Border { color: ThemeColors::DANGER, width: 1.0, radius: 6.0.into() },
                ..Default::default()
            });
            
        rows_col = rows_col.push(card);
    }

    let header = text("Detected Anomalies").size(18).style(|_| text::Style { color: Some(ThemeColors::DANGER) });
    let subtitle = text("These processes show suspicious behavior based on heuristics.").size(13).style(|_| text::Style { color: Some(ThemeColors::TEXT_MUTED) });

    container(
        scrollable(
            column![
                header,
                subtitle,
                Space::with_height(Length::Fixed(16.0)),
                rows_col
            ].spacing(4).padding(20)
        )
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
