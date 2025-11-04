#[derive(Debug, Clone, Copy)]
pub struct Icons {
    pub check: &'static str,

    pub cross: &'static str,

    pub bullet: &'static str,

    pub arrow_right: &'static str,

    pub warning: &'static str,

    pub info: &'static str,

    pub hourglass: &'static str,

    pub search: &'static str,

    pub backspace: &'static str,

    pub enter: &'static str,

    pub page_up: &'static str,

    pub page_down: &'static str,

    pub block_filled: &'static str,

    pub block_empty: &'static str,
}

impl Icons {
    pub const UNICODE: Self = Self {
        check: "✓",
        cross: "✗",
        bullet: "●",
        arrow_right: "▶",
        warning: "⚠",
        info: "ⓘ",
        hourglass: "⌛",
        search: "🔍",
        backspace: "⌫",
        enter: "⏎",
        page_up: "⇞",
        page_down: "⇟",
        block_filled: "█",
        block_empty: "░",
    };
}

impl Default for Icons {
    fn default() -> Self {
        Self::UNICODE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_set_complete() {
        let icons = Icons::UNICODE;
        assert_eq!(icons.check, "✓");
        assert_eq!(icons.cross, "✗");
        assert_eq!(icons.bullet, "●");
        assert_eq!(icons.arrow_right, "▶");
        assert_eq!(icons.warning, "⚠");
        assert_eq!(icons.info, "ⓘ");
        assert_eq!(icons.hourglass, "⌛");
        assert_eq!(icons.search, "🔍");
        assert_eq!(icons.backspace, "⌫");
        assert_eq!(icons.enter, "⏎");
        assert_eq!(icons.page_up, "⇞");
        assert_eq!(icons.page_down, "⇟");
        assert_eq!(icons.block_filled, "█");
        assert_eq!(icons.block_empty, "░");
    }

    #[test]
    fn test_default_is_unicode() {
        let icons = Icons::default();
        assert_eq!(icons.check, Icons::UNICODE.check);
        assert_eq!(icons.arrow_right, Icons::UNICODE.arrow_right);
    }

    #[test]
    fn test_all_icons_non_empty() {
        let icons = Icons::UNICODE;
        assert!(!icons.check.is_empty());
        assert!(!icons.cross.is_empty());
        assert!(!icons.bullet.is_empty());
        assert!(!icons.arrow_right.is_empty());
        assert!(!icons.warning.is_empty());
        assert!(!icons.info.is_empty());
        assert!(!icons.hourglass.is_empty());
        assert!(!icons.search.is_empty());
        assert!(!icons.backspace.is_empty());
        assert!(!icons.enter.is_empty());
        assert!(!icons.page_up.is_empty());
        assert!(!icons.page_down.is_empty());
        assert!(!icons.block_filled.is_empty());
        assert!(!icons.block_empty.is_empty());
    }
}
