use ratatui::{buffer::Buffer, layout::Rect};

use super::UiCtx;

/// View trait for rendering UI components with props-based rendering.
///
/// Uses a generic associated type for Props to preserve lifetimes
/// without unsafe. Implementations can bind Props to borrowed data
/// (like slices) from the app model.
///
/// Invariants
/// - render() is pure: same props + ctx + area => same buffer output
/// - render() does not mutate props or ctx
/// - render() handles all prop states
pub trait View {
    type Props<'a>;

    fn render<'a>(&self, props: &Self::Props<'a>, ctx: &UiCtx, area: Rect, buf: &mut Buffer);
}
