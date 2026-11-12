use gpui_kit::base::StyledExt as _;
use gpui_kit::component::empty::{
    EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use gpui_kit::component::{ActiveTheme as _, Icon, IconName, Theme, TitleBar};
use gpui_kit::{
    Context, IntoElement, ParentElement as _, Render, Styled as _, Subscription, Window, div,
};

use crate::APP_NAME;

/// The window shell: a title bar above the workspace.
///
/// Product views belong in [`MainView::render_workspace`], which currently
/// shows the empty state because no view exists yet.
pub struct MainView {
    /// Keeps the appearance observer alive; dropping it stops the tracking.
    _appearance: Subscription,
}

impl MainView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Open in the desktop's appearance, then follow it while the app runs.
        Theme::change(window.appearance(), None, cx);
        let appearance = window.observe_window_appearance(|window, cx| {
            Theme::change(window.appearance(), None, cx);
        });

        Self {
            _appearance: appearance,
        }
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(TitleBar::new().child(div().text_sm().child(APP_NAME)))
            .child(self.render_workspace(cx))
    }
}

impl MainView {
    /// The region that product views replace.
    ///
    /// It owns the remaining height and nothing else: a view placed here
    /// decides its own scrolling, toolbars, and panes.
    fn render_workspace(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .flex_1()
            .min_h_0()
            .items_center()
            .justify_center()
            .p_6()
            .child(self.render_empty_state(cx))
    }

    /// Stand-in for the first view. Replace it rather than extending it.
    fn render_empty_state(&self, _cx: &Context<Self>) -> impl IntoElement {
        EmptyHeader::new()
            .media(
                EmptyMedia::new()
                    .with_variant(EmptyMediaVariant::Icon)
                    .child(Icon::new(IconName::Network)),
            )
            .title(EmptyTitle::new().child("Nothing to show yet"))
            .description(EmptyDescription::new().child("No content in this window."))
    }
}
