use crate::ui::icons::Icons;
use crate::ui::theme::ThemeVariant;

/// UI context holding shared theme and icons.
/// Passed to all view render methods.
pub struct UiCtx {
    pub theme: ThemeVariant,
    pub icons: Icons,
}

impl UiCtx {
    pub fn new() -> Self {
        Self {
            theme: ThemeVariant::default(),
            icons: Icons::UNICODE,
        }
    }
}

impl Default for UiCtx {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_ctx_default() {
        let ctx = UiCtx::default();
        assert_eq!(ctx.icons.check, Icons::UNICODE.check);
    }

    #[test]
    fn test_ui_ctx_new() {
        let ctx = UiCtx::new();
        assert_eq!(ctx.icons.arrow_right, Icons::UNICODE.arrow_right);
    }
}
