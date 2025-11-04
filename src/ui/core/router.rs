use ratatui::{buffer::Buffer, layout::Rect};

use crate::app::state::{Model, Screen};

use super::props::{ConfirmProps, DeviceSelectProps, DoneProps, IsoSearchProps, WritingProps};
use super::{UiCtx, View};
use crate::ui::screens::views::{
    ConfirmScreen, DeviceSelectScreen, DoneScreen, IsoSearchScreen, WritingScreen,
};
use crate::ui::widgets::{AppShell, app_shell::compute_border_color};

/// Router responsible for dispatching Model to the appropriate screen renderer.
///
/// Extracts Props from Model and delegates to View trait implementations.
pub struct UiRouter {
    pub ctx: UiCtx,
}

impl UiRouter {
    pub fn new(ctx: UiCtx) -> Self {
        Self { ctx }
    }

    /// Projects Model → Props, then calls View::render() for the active screen
    pub fn render(&self, model: &Model, area: Rect, buf: &mut Buffer) {
        let shell = AppShell {
            title: " ferric ",
            border_color: compute_border_color(model),
        };

        shell.render(model, self, area, buf);
    }

    /// Renders a specific screen through Props projection
    pub(crate) fn render_screen(&self, model: &Model, area: Rect, buf: &mut Buffer) {
        match model.screen {
            Screen::IsoSearch => {
                let props = IsoSearchProps::from_model(model);
                IsoSearchScreen.render(&props, &self.ctx, area, buf);
            }
            Screen::DeviceSelect => {
                let props = DeviceSelectProps::from_model(model);
                DeviceSelectScreen.render(&props, &self.ctx, area, buf);
            }
            Screen::Confirm => {
                let props = ConfirmProps::from_model(model);
                ConfirmScreen.render(&props, &self.ctx, area, buf);
            }
            Screen::Writing => {
                let props = WritingProps::from_model(model);
                WritingScreen.render(&props, &self.ctx, area, buf);
            }
            Screen::Done => {
                let props = DoneProps::from_model(model);
                DoneScreen.render(&props, &self.ctx, area, buf);
            }
        }
    }
}

impl Default for UiRouter {
    fn default() -> Self {
        Self::new(UiCtx::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_default() {
        let router = UiRouter::default();
        assert_eq!(router.ctx.icons.check, "✓");
    }

    #[test]
    fn test_router_new() {
        let ctx = UiCtx::new();
        let router = UiRouter::new(ctx);
        assert_eq!(router.ctx.icons.arrow_right, "▶");
    }

    #[test]
    fn test_router_renders_all_screens() {
        let router = UiRouter::default();
        let area = Rect::new(0, 0, 120, 40);
        let mut buf = Buffer::empty(area);

        // IsoSearch
        let mut model = Model::default();
        model.screen = Screen::IsoSearch;
        router.render(&model, area, &mut buf);

        // DeviceSelect
        model.screen = Screen::DeviceSelect;
        router.render(&model, area, &mut buf);

        // Confirm
        model.screen = Screen::Confirm;
        router.render(&model, area, &mut buf);

        // Writing
        model.screen = Screen::Writing;
        router.render(&model, area, &mut buf);

        // Done
        model.screen = Screen::Done;
        router.render(&model, area, &mut buf);
    }
}
