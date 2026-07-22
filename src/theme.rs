use iced::Color;

pub struct ThemeColors;

impl ThemeColors {
    pub const BACKGROUND: Color = Color::from_rgb(0.06, 0.09, 0.16); // #0f172a
    pub const SIDEBAR_BG: Color = Color::from_rgb(0.12, 0.16, 0.23); // #1e293b
    pub const CARD_BG: Color = Color::from_rgb(0.12, 0.16, 0.23);
    pub const CARD_HOVER: Color = Color::from_rgb(0.20, 0.25, 0.33);
    pub const CARD_BORDER: Color = Color::from_rgb(0.20, 0.25, 0.33);
    
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.97, 0.98, 0.99);
    pub const TEXT_MUTED: Color = Color::from_rgb(0.58, 0.64, 0.72);

    pub const CPU_ACCENT: Color = Color::from_rgb(0.02, 0.71, 0.83);    // Cyan (#06b6d4)
    pub const RAM_ACCENT: Color = Color::from_rgb(0.66, 0.33, 0.97);    // Purple (#a855f7)
    pub const DISK_ACCENT: Color = Color::from_rgb(0.06, 0.73, 0.51);   // Emerald (#10b981)
    pub const NET_ACCENT: Color = Color::from_rgb(0.96, 0.62, 0.04);    // Amber (#f59e0b)
    
    pub const DANGER: Color = Color::from_rgb(0.94, 0.27, 0.27);        // Red (#ef4444)
    #[allow(dead_code)]
    pub const SUCCESS: Color = Color::from_rgb(0.13, 0.77, 0.36);       // Green (#22c55e)
}
