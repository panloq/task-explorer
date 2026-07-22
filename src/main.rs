mod app;
mod components;
mod config;
mod lang;
mod system;
mod theme;
mod views;
mod startup;

use app::TaskExplorerApp;

pub fn main() -> iced::Result {
    iced::application(
        TaskExplorerApp::title,
        TaskExplorerApp::update,
        TaskExplorerApp::view,
    )
    .subscription(TaskExplorerApp::subscription)
    .theme(TaskExplorerApp::theme)
    .window(iced::window::Settings {
        size: iced::Size::new(1200.0, 740.0),
        min_size: Some(iced::Size::new(900.0, 580.0)),
        icon: iced::window::icon::from_file_data(include_bytes!("../assets/app_icon.png"), None).ok(),
        ..Default::default()
    })
    .run_with(TaskExplorerApp::new)
}
