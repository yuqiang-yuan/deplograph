use gpui_kit::assets::IconName;
use gpui_kit::base::{Placement, StyledExt as _};
use gpui_kit::component::empty::{
    EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use gpui_kit::component::{
    ActiveTheme as _, Icon, TitleBar,
    button::{Button, ButtonVariants as _},
    menu::{DropdownMenu as _, PopupMenuItem},
};
use gpui_kit::{App, Context, IntoElement, ParentElement as _, Render, Styled as _, Window, div};

use crate::appearance;
use crate::globals::APP_NAME;
use crate::settings::{AppSettings, FONT_SIZES};
use crate::settings_dialog;

/// 窗口外壳：标题栏 + 一屏内容。
///
/// 产品视图放在 [`MainView::render_workspace`]，现在还没有，所以那里是空态。
pub struct MainView;

impl MainView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }

    /// 标题栏：左边是应用标记，右边是字号、主题、设置三个控件。
    fn render_title_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        TitleBar::new().child(
            div()
                .size_full()
                .h_flex()
                .items_center()
                .gap_1()
                .child(app_mark(cx))
                .child(div().flex_grow_1())
                .child(font_size_button())
                .child(theme_button(cx))
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

/// 字号菜单：Small / Regular / Large，选完立刻生效并落盘。
fn font_size_button() -> impl IntoElement {
    Button::new("font-size-button")
        .ghost()
        .rounded_none()
        .icon(IconName::ALargeSmall)
        .tooltip("Font size")
        .tooltip_placement(Placement::Bottom)
        .dropdown_menu(|menu, _, cx| {
            let current = cx.global::<AppSettings>().font_size();

            FONT_SIZES.iter().fold(menu, |menu, (size, label)| {
                let size = *size;
                menu.item(
                    PopupMenuItem::new(*label)
                        .checked(current == size)
                        .on_click(move |_, _, cx| appearance::set_font_size(size, cx)),
                )
            })
        })
}

/// 明暗切换按钮：当前是深色就显示月亮，点一下切到另一个。
fn theme_button(cx: &App) -> impl IntoElement {
    let is_dark = cx.theme().is_dark();

    Button::new("theme-button")
        .ghost()
        .rounded_none()
        .icon(if is_dark {
            IconName::Moon
        } else {
            IconName::Sun
        })
        .tooltip(if is_dark {
            "Switch to light theme"
        } else {
            "Switch to dark theme"
        })
        .tooltip_placement(Placement::Bottom)
        .on_click(|_, _, cx| appearance::set_dark_mode(!cx.theme().is_dark(), cx))
}

/// 设置按钮：打开设置弹窗（保存和 Esc 取消都在弹窗里）。
fn settings_button() -> impl IntoElement {
    Button::new("settings-button")
        .ghost()
        .rounded_none()
        .icon(IconName::Settings)
        .tooltip("Settings")
        .tooltip_placement(Placement::Bottom)
        .on_click(|_, window, cx| settings_dialog::open(window, cx))
}
