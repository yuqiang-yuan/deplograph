use gpui_kit::base::StyledExt as _;
use gpui_kit::component::dialog::DialogButtonProps;
use gpui_kit::component::radio::RadioGroup;
use gpui_kit::component::switch::Switch;
use gpui_kit::component::{ActiveTheme as _, WindowExt as _};
use gpui_kit::{
    App, AppContext as _, Context, Hsla, IntoElement, ParentElement as _, Render, Styled as _,
    Window, div,
};

use crate::appearance;
use crate::settings::{AppSettings, FONT_SIZES};

/// 打开设置弹窗。
///
/// 弹窗里改的是一份草稿：点「保存」才连同主题一起生效，「关闭」和 Esc 直接丢掉。
pub fn open(window: &mut Window, cx: &mut App) {
    // 实体要先建好：弹窗层的构建闭包可能被重跑，不能在闭包里建
    let form = cx.new(SettingsForm::new);

    window.open_dialog(cx, move |dialog, _, _| {
        let form = form.clone();

        dialog
            .title("Settings")
            .child(form.clone())
            // 底部两个按钮：左边「关闭」（等同 Esc），右边「保存」
            .button_props(
                DialogButtonProps::default()
                    .show_cancel(true)
                    .ok_text("Save")
                    .cancel_text("Close"),
            )
            .on_ok(move |_, _, cx| {
                // 先取出草稿：read() 借着 cx，apply() 要可变借用
                let draft = form.read(cx).draft.clone();
                appearance::apply(&draft, cx);
                // true = 保存后关掉弹窗
                true
            })
    });
}

/// 弹窗里的表单：改的是草稿，保存时才由 [`appearance::apply`] 落到主题和设置文件。
struct SettingsForm {
    draft: AppSettings,
}

impl SettingsForm {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            draft: cx.global::<AppSettings>().clone(),
        }
    }

    fn render_dark_mode(&self, cx: &mut Context<Self>) -> impl IntoElement {
        setting_row(
            cx.theme().muted_foreground,
            "Dark mode",
            "Use the dark theme.",
            Switch::new("settings-dark-mode")
                .checked(self.draft.is_dark())
                .on_change(cx.listener(|this, is_dark, _, cx| {
                    this.draft.is_dark = Some(*is_dark);
                    cx.notify();
                })),
        )
    }

    fn render_font_size(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = FONT_SIZES
            .iter()
            .position(|(size, _)| *size == self.draft.font_size());

        setting_row(
            cx.theme().muted_foreground,
            "Font size",
            "Text size for the whole window.",
            RadioGroup::horizontal("settings-font-size")
                .flex_none()
                .children(FONT_SIZES.iter().map(|(_, label)| *label))
                .selected_index(selected)
                .on_change(cx.listener(|this, index, _, cx| {
                    if let Some((size, _)) = FONT_SIZES.get(*index) {
                        this.draft.font_size = Some(*size);
                        cx.notify();
                    }
                })),
        )
    }
}

impl Render for SettingsForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_5()
            .child(self.render_dark_mode(cx))
            .child(self.render_font_size(cx))
    }
}

/// 一行设置：左边标题和说明，右边控件。
fn setting_row(
    muted: Hsla,
    title: &'static str,
    description: &'static str,
    control: impl IntoElement,
) -> impl IntoElement {
    div()
        .h_flex()
        .items_center()
        .justify_between()
        .gap_4()
        .child(
            div()
                .flex_1()
                .min_w_0()
                .v_flex()
                .gap_1()
                .child(div().child(title))
                .child(div().text_sm().text_color(muted).child(description)),
        )
        .child(control)
}
