use gpui_kit::assets::IconName;
use gpui_kit::base::{Placement, StyledExt as _};
use gpui_kit::component::empty::{
    EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use gpui_kit::component::{
    ActiveTheme as _, Icon, TitleBar,
    button::{Button, ButtonVariants as _},
};
use gpui_kit::{
    App, Context, InteractiveElement as _, IntoElement, MouseButton, ParentElement as _, Render,
    Styled as _, Window, div,
};

use crate::globals::APP_NAME;
use crate::settings_dialog;

/// 窗口外壳：标题栏 + 一屏内容。
///
/// 产品视图放在 [`MainView::render_workspace`]，现在还没有，所以那里是空态。
pub struct MainView;

impl MainView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }

    /// 标题栏：左边是应用标记，右边只有设置（字号、明暗这些都在设置弹窗里）。
    fn render_title_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new().child(
            div()
                .size_full()
                .h_flex()
                .items_center()
                .gap_1()
                .child(app_mark(cx))
                .child(div().flex_grow_1())
                .child(settings_button()),
        )
    }

    /// 产品视图要替换掉的那块区域。
    ///
    /// 它只占满剩余高度，别的不管：放进去的视图自己决定滚动、工具栏和分栏。
    fn render_workspace(&self, _cx: &Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .flex_1()
            .min_h_0()
            .items_center()
            .justify_center()
            .p_6()
            .child(
                EmptyHeader::new()
                    .media(
                        EmptyMedia::new()
                            .with_variant(EmptyMediaVariant::Icon)
                            .child(Icon::new(IconName::Network)),
                    )
                    .title(EmptyTitle::new().child("Nothing to show yet"))
                    .description(EmptyDescription::new().child("No content in this window.")),
            )
    }
}

impl Render for MainView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.render_title_bar(cx))
            .child(self.render_workspace(cx))
    }
}

/// 应用标记：自己的 SVG（assets/icons/deplograph.svg）加名字。
fn app_mark(cx: &App) -> impl IntoElement {
    div()
        .h_flex()
        .items_center()
        .gap_1()
        .child(
            // 路径交给 AppAssets 解析，也就是 assets/ 目录
            Icon::default()
                .path("icons/deplograph.svg")
                .size_4()
                .text_color(cx.theme().primary),
        )
        .child(div().text_sm().child(APP_NAME))
}

/// 设置按钮：打开设置弹窗（保存和 Esc 取消都在弹窗里）。
///
/// 外面套一层 div 专门挡掉「左键按下」的冒泡，否则点击会被窗口拖动吃掉：
/// 标题栏在左键按下时把 `should_move` 置真，之后指针只要动一下就 `start_window_move()`
/// 去拖窗口，这一次点击就没了 —— 表现就是「有时能弹出、有时点不动」。
/// gpui-component 自己的最小化/最大化/关闭按钮也是这么挡的（title_bar.rs 里的 on_mouse_down）。
/// 点击本身不用管：Button 内部已经 `stop_propagation`，不会冒到标题栏的双击最大化。
fn settings_button() -> impl IntoElement {
    div()
        .on_mouse_down(MouseButton::Left, |_, window, cx| {
            window.prevent_default();
            cx.stop_propagation();
        })
        .child(
            Button::new("settings-button")
                .ghost()
                .rounded_none()
                .icon(IconName::Settings)
                .tooltip(rust_i18n::t!("app.settings").to_string())
                .tooltip_placement(Placement::Bottom)
                .on_click(|_, window, cx| settings_dialog::open(window, cx)),
        )
}
