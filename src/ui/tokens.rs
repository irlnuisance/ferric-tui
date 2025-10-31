use ratatui::symbols::border;
use std::env;

/// spacing const
pub const S0: u16 = 0;

pub const S1: u16 = 1;

pub const S2: u16 = 2;

pub const S3: u16 = 3;

pub const NARROW_WIDTH: u16 = 80;

pub const WIDE_WIDTH: u16 = 140;

pub mod borders {
    use ratatui::symbols::border;

    pub const ROUNDED: border::Set = border::ROUNDED;

    pub const DOUBLE: border::Set = border::DOUBLE;

    pub const PLAIN: border::Set = border::PLAIN;

    pub const ROUNDED_ASCII: border::Set = border::PLAIN;

    pub const DOUBLE_ASCII: border::Set = border::PLAIN;

    pub const PLAIN_ASCII: border::Set = border::PLAIN;
}

#[derive(Debug, Clone)]
pub struct UiCapabilities {
    pub unicode: bool,
}

impl UiCapabilities {
    pub fn detect() -> Self {
        if env::var("FERRIC_ASCII").is_ok_and(|v| v == "1") {
            return Self { unicode: false };
        }

        if let Ok(term) = env::var("TERM") {
            let term_lower = term.to_lowercase();
            if term_lower.contains("linux") || term_lower.contains("vt100") {
                return Self { unicode: false };
            }
        }

        let locale = env::var("LC_ALL")
            .or_else(|_| env::var("LANG"))
            .unwrap_or_default()
            .to_uppercase();

        if locale.contains("UTF") {
            return Self { unicode: true };
        }

        Self { unicode: true }
    }

    pub fn border_default(&self) -> border::Set {
        if self.unicode {
            borders::ROUNDED
        } else {
            borders::ROUNDED_ASCII
        }
    }

    pub fn border_focused(&self) -> border::Set {
        if self.unicode {
            borders::DOUBLE
        } else {
            borders::DOUBLE_ASCII
        }
    }

    pub fn border_muted(&self) -> border::Set {
        if self.unicode {
            borders::PLAIN
        } else {
            borders::PLAIN_ASCII
        }
    }
}

impl Default for UiCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_constants() {
        assert_eq!(S0, 0);
        assert_eq!(S1, 1);
        assert_eq!(S2, 2);
        assert_eq!(S3, 3);
    }

    #[test]
    fn test_responsive_thresholds() {
        assert_eq!(NARROW_WIDTH, 80);
        assert_eq!(WIDE_WIDTH, 140);
    }

    #[test]
    fn test_capability_detection_from_env() {
        unsafe {
            env::set_var("FERRIC_ASCII", "1");
        }
        let caps = UiCapabilities::detect();
        assert!(!caps.unicode, "FERRIC_ASCII=1 should disable Unicode");
        unsafe {
            env::remove_var("FERRIC_ASCII");
        }

        unsafe {
            env::remove_var("TERM");
            env::remove_var("LC_ALL");
            env::remove_var("LANG");
        }
        let caps = UiCapabilities::detect();
        assert!(caps.unicode, "Default should enable Unicode");
    }

    #[test]
    fn test_border_set_ascii_fallback() {
        let caps = UiCapabilities { unicode: false };
        let border = caps.border_default();
        assert_eq!(border.top_left, borders::PLAIN_ASCII.top_left);
    }

    #[test]
    fn test_border_set_unicode() {
        let caps = UiCapabilities { unicode: true };
        let border = caps.border_default();
        assert_eq!(border.top_left, borders::ROUNDED.top_left);
    }

    #[test]
    fn test_border_focused_emphasis() {
        let caps_unicode = UiCapabilities { unicode: true };
        let caps_ascii = UiCapabilities { unicode: false };

        let unicode_focused = caps_unicode.border_focused();
        let _ascii_focused = caps_ascii.border_focused();

        assert_ne!(
            unicode_focused.top_left,
            caps_unicode.border_default().top_left
        );
    }
}
