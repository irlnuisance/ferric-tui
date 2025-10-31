#![allow(dead_code)]

pub mod colors {
    use ratatui::style::Color;

    pub const PRIMARY: Color = Color::Rgb(255, 140, 60);
    pub const PRIMARY_DIM: Color = Color::Rgb(180, 90, 40);
    pub const SECONDARY: Color = Color::Rgb(160, 170, 180);
    pub const ACCENT: Color = Color::Rgb(220, 80, 60);

    pub const SUCCESS: Color = Color::Rgb(140, 160, 170);
    pub const SUCCESS_DIM: Color = Color::Rgb(100, 115, 125);
    pub const WARNING: Color = Color::Rgb(255, 165, 80);
    pub const DANGER: Color = Color::Rgb(200, 60, 50);
    pub const DANGER_BRIGHT: Color = Color::Rgb(240, 70, 50);

    pub const TEXT_PRIMARY: Color = Color::Rgb(235, 230, 225);
    pub const TEXT_SECONDARY: Color = Color::Rgb(160, 155, 150);
    pub const TEXT_DIM: Color = Color::Rgb(110, 105, 100);

    pub const SELECTED_BG: Color = Color::Rgb(70, 40, 30);
    pub const SELECTED_FG: Color = Color::Rgb(255, 255, 255);
    pub const HOVER_BG: Color = Color::Rgb(40, 35, 35);

    pub const BG_PRIMARY: Color = Color::Rgb(18, 16, 16);
    pub const BG_SECONDARY: Color = Color::Rgb(30, 28, 26);
    pub const BG_TERTIARY: Color = Color::Rgb(42, 40, 38);

    pub const BORDER_ACTIVE: Color = PRIMARY;
    pub const BORDER_INACTIVE: Color = Color::Rgb(80, 75, 70);
    pub const BORDER_FOCUS: Color = ACCENT;

    pub const NEUTRAL_50: Color = Color::Rgb(235, 230, 225);
    pub const NEUTRAL_100: Color = Color::Rgb(210, 205, 200);
    pub const NEUTRAL_200: Color = Color::Rgb(185, 180, 175);
    pub const NEUTRAL_300: Color = Color::Rgb(160, 155, 150);
    pub const NEUTRAL_400: Color = Color::Rgb(135, 130, 125);
    pub const NEUTRAL_500: Color = Color::Rgb(110, 105, 100);
    pub const NEUTRAL_600: Color = Color::Rgb(85, 80, 75);
    pub const NEUTRAL_700: Color = Color::Rgb(60, 55, 50);
    pub const NEUTRAL_800: Color = Color::Rgb(42, 40, 38);
    pub const NEUTRAL_900: Color = Color::Rgb(30, 28, 26);
    pub const NEUTRAL_950: Color = Color::Rgb(18, 16, 16);

    pub fn success_fg() -> Color {
        SUCCESS
    }

    pub fn success_bg() -> Color {
        BG_TERTIARY
    }

    pub fn warning_fg() -> Color {
        WARNING
    }

    pub fn warning_bg() -> Color {
        Color::Rgb(55, 45, 30)
    }

    pub fn danger_fg() -> Color {
        DANGER_BRIGHT
    }

    pub fn danger_bg() -> Color {
        Color::Rgb(60, 25, 20)
    }

    pub fn info_fg() -> Color {
        SECONDARY
    }

    pub fn info_bg() -> Color {
        BG_TERTIARY
    }
}

pub mod styles {
    use super::colors;
    use ratatui::style::{Modifier, Style};

    pub fn title() -> Style {
        Style::default()
            .fg(colors::PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    pub fn subtitle() -> Style {
        Style::default()
            .fg(colors::TEXT_SECONDARY)
            .add_modifier(Modifier::DIM)
    }

    pub fn text() -> Style {
        Style::default().fg(colors::TEXT_PRIMARY)
    }

    pub fn text_muted() -> Style {
        Style::default().fg(colors::TEXT_SECONDARY)
    }

    pub fn text_dim() -> Style {
        Style::default()
            .fg(colors::TEXT_DIM)
            .add_modifier(Modifier::DIM)
    }

    pub fn selected() -> Style {
        Style::default()
            .fg(colors::SELECTED_FG)
            .bg(colors::SELECTED_BG)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected_marker() -> Style {
        Style::default()
            .fg(colors::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn success() -> Style {
        Style::default()
            .fg(colors::SUCCESS)
            .add_modifier(Modifier::BOLD)
    }

    pub fn warning() -> Style {
        Style::default()
            .fg(colors::WARNING)
            .add_modifier(Modifier::BOLD)
    }

    pub fn danger() -> Style {
        Style::default()
            .fg(colors::DANGER_BRIGHT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn highlight() -> Style {
        Style::default()
            .fg(colors::ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn code() -> Style {
        Style::default()
            .fg(colors::SECONDARY)
            .add_modifier(Modifier::ITALIC)
    }

    pub fn emphasis() -> Style {
        Style::default()
            .fg(colors::PRIMARY)
            .add_modifier(Modifier::ITALIC)
    }

    pub fn border_focused() -> Style {
        Style::default().fg(colors::BORDER_FOCUS)
    }

    pub fn border_default() -> Style {
        Style::default().fg(colors::BORDER_ACTIVE)
    }

    pub fn border_muted() -> Style {
        Style::default().fg(colors::BORDER_INACTIVE)
    }

    pub fn success_bg() -> Style {
        Style::default()
            .fg(colors::success_fg())
            .bg(colors::success_bg())
    }

    pub fn warning_bg() -> Style {
        Style::default()
            .fg(colors::warning_fg())
            .bg(colors::warning_bg())
    }

    pub fn danger_bg() -> Style {
        Style::default()
            .fg(colors::danger_fg())
            .bg(colors::danger_bg())
    }

    pub fn info_bg() -> Style {
        Style::default().fg(colors::info_fg()).bg(colors::info_bg())
    }
}

use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct ThemeVariant {
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_dim: Color,
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub bg_tertiary: Color,
    pub border_active: Color,
    pub border_inactive: Color,
    pub border_focus: Color,
    pub primary: Color,
    pub accent: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
}

impl ThemeVariant {
    pub fn dark() -> Self {
        Self {
            text_primary: colors::TEXT_PRIMARY,
            text_secondary: colors::TEXT_SECONDARY,
            text_dim: colors::TEXT_DIM,
            bg_primary: colors::BG_PRIMARY,
            bg_secondary: colors::BG_SECONDARY,
            bg_tertiary: colors::BG_TERTIARY,
            border_active: colors::BORDER_ACTIVE,
            border_inactive: colors::BORDER_INACTIVE,
            border_focus: colors::BORDER_FOCUS,
            primary: colors::PRIMARY,
            accent: colors::ACCENT,
            success: colors::SUCCESS,
            warning: colors::WARNING,
            danger: colors::DANGER_BRIGHT,
        }
    }

    pub fn high_contrast() -> Self {
        Self {
            text_primary: Color::Rgb(255, 255, 255),    // Pure white
            text_secondary: Color::Rgb(200, 200, 200),  // Light gray
            text_dim: Color::Rgb(150, 150, 150),        // Medium gray
            bg_primary: Color::Rgb(0, 0, 0),            // Pure black
            bg_secondary: Color::Rgb(20, 20, 20),       // Near black
            bg_tertiary: Color::Rgb(40, 40, 40),        // Dark gray
            border_active: Color::Rgb(255, 255, 255),   // White borders
            border_inactive: Color::Rgb(100, 100, 100), // Gray borders
            border_focus: Color::Rgb(255, 200, 0),      // Bright yellow
            primary: Color::Rgb(255, 180, 80),          // Brighter orange
            accent: Color::Rgb(255, 100, 100),          // Brighter red
            success: Color::Rgb(100, 255, 100),         // Bright green
            warning: Color::Rgb(255, 220, 0),           // Bright yellow
            danger: Color::Rgb(255, 80, 80),            // Bright red
        }
    }
}

impl Default for ThemeVariant {
    fn default() -> Self {
        Self::dark()
    }
}
