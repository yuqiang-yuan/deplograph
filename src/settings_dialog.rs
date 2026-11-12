use gpui_kit::base::StyledExt as _;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::dialog::{DialogAction, DialogClose, DialogFooter};
use gpui_kit::component::radio::RadioGroup;
use gpui_kit::component::select::{Select, SelectState};
use gpui_kit::component::{ActiveTheme as _, IndexPath, WindowExt as _};
use gpui_kit::{
    App, AppContext as _, Context, Entity, Hsla, IntoElement, ParentElement as _, Render,
    SharedString, Styled as _, Window, div, px,
};

use crate::appearance;
use crate::i18n;
use crate::settings::{AppSettings, FONT_SIZES};

/// 打开设置弹窗。
///
/// 弹窗里改的是一份草稿：点「保存」才连同主题一起生效，「关闭」和 Esc 直接丢掉。
pub fn open(window: &mut Window, cx: &mut App) {
    // 实体要先建好：弹窗层的构建闭包可能被重跑，不能在闭包里建
    let form = cx.new(|cx| SettingsForm::new(window, cx));

    window.open_dialog(cx, move |dialog, _, _| {
        let form = form.clone();

        dialog
            .title(rust_i18n::t!("app.settings").to_string())
            .w(px(600.0))
            .child(form.clone())
            // 普通 Dialog 不渲染 button_props 里的按钮（那条路只服务 AlertDialog 的默认
            // footer），所以底部按钮得自己拼：DialogAction 点下去派发 Confirm（走 on_ok），
            // DialogClose 派发 Cancel（默认回调就是关窗）。文案每次重建时取，跟着当前语言走。
            .footer(
                DialogFooter::new()
                    .child(
                        DialogClose::new()
                            .trigger(|button| button.label(rust_i18n::t!("app.close").to_string())),
                    )
                    .child(
                        DialogAction::new().child(
                            Button::new("settings-save")
                                .label(rust_i18n::t!("app.save").to_string())
                                .primary(),
                        ),
                    ),
            )
            .on_ok(move |_, _, cx| {
                // 先把草稿取出来（顺手把下拉的当前选择折进 language）：
                // read() 借着 cx，下面 apply() 要可变借用，所以让借用在这一块里结束
                let draft = {
                    let form = form.read(cx);
                    let mut draft = form.draft.clone();
                    if let Some(code) = form.selected_language(cx) {
                        draft.language = Some(code.to_string());
                    }
                    draft
                };

                appearance::apply(&draft, cx);
                i18n::set_locale(draft.language(), cx);
                // true = 保存后关掉弹窗
                true
            })
    });
}

/// 弹窗里的表单：改的是草稿，保存时才由 [`appearance::apply`] 落到主题、设置文件。
struct SettingsForm {
    draft: AppSettings,
    /// 语言下拉：选项是各语言自己的名字（English / 简体中文 / 繁體中文），
    /// 选中项只是下标，保存时用 [`SettingsForm::selected_language`] 映射回 locale 码。
    language: Entity<SelectState<Vec<SharedString>>>,
}

impl SettingsForm {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let draft = cx.global::<AppSettings>().clone();

        let items = i18n::SUPPORTED_LOCALES
            .iter()
            .map(|(_, label)| SharedString::from(*label))
            .collect::<Vec<_>>();
        let selected = i18n::SUPPORTED_LOCALES
            .iter()
            .position(|(code, _)| *code == draft.language())
            .map(IndexPath::new);
        let language = cx.new(|cx| SelectState::new(items, selected, window, cx));

        Self { draft, language }
    }

    /// 下拉的当前选择 → locale 码（下标对应 [`i18n::SUPPORTED_LOCALES`]）。
    fn selected_language(&self, cx: &App) -> Option<&'static str> {
        let row = self.language.read(cx).selected_index(cx)?.row;
        i18n::SUPPORTED_LOCALES.get(row).map(|(code, _)| *code)
    }

    /// 主题：亮色 / 暗色两个单选（下标 0 = 亮色，1 = 暗色）。
    fn render_theme(&self, cx: &mut Context<Self>) -> impl IntoElement {
        setting_row(
            cx.theme().muted_foreground,
            rust_i18n::t!("app.theme").to_string(),
            rust_i18n::t!("app.theme_description").to_string(),
            RadioGroup::horizontal("settings-theme")
                .flex_none()
                .children([
                    rust_i18n::t!("app.theme_light").to_string(),
                    rust_i18n::t!("app.theme_dark").to_string(),
                ])
                .selected_index(Some(usize::from(self.draft.is_dark())))
                .on_change(cx.listener(|this, index: &usize, _, cx| {
                    this.draft.is_dark = Some(*index == 1);
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
            rust_i18n::t!("app.font_size").to_string(),
            rust_i18n::t!("app.font_size_description").to_string(),
            // 档位文案的 key 存在 FONT_SIZES 里；t! 支持运行时 key，直接查这张表
            RadioGroup::horizontal("settings-font-size")
                .flex_none()
                .children(
                    FONT_SIZES
                        .iter()
                        .map(|(_, key)| rust_i18n::t!(*key).to_string()),
                )
                .selected_index(selected)
                .on_change(cx.listener(|this, index: &usize, _, cx| {
                    if let Some(&(size, _)) = FONT_SIZES.get(*index) {
                        this.draft.font_size = Some(size);
                        cx.notify();
                    }
                })),
        )
    }

    fn render_language(&self, cx: &mut Context<Self>) -> impl IntoElement {
        setting_row(
            cx.theme().muted_foreground,
            rust_i18n::t!("app.language").to_string(),
            rust_i18n::t!("app.language_description").to_string(),
            // 选项名不走 t!：语言名一律以该语言自己显示，所以这里不翻译。
            //
            // Select 得塞进一个定宽 div 里：它 render 出来的根节点是 `size_full()`
            // （select.rs 里 `BaseSelect::new(id)….size_full()`），在横向 flex 里会把整行
            // 吃满、把左边的标题挤掉；直接写 `Select::…w(px(..))` 也不行 —— 那个宽度进的是
            // Select 自己的 options.style（内层），管不到根节点。
            div().w(px(180.0)).child(Select::new(&self.language)),
        )
    }
}

impl Render for SettingsForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_5()
            .child(self.render_theme(cx))
            .child(self.render_font_size(cx))
            .child(self.render_language(cx))
    }
}

/// 一行设置：左边标题和说明，右边控件。
///
/// 标题和说明收 `impl IntoElement` 而不是 `&'static str`，因为它们是 `t!` 出来的（String）。
fn setting_row(
    muted: Hsla,
    title: impl IntoElement,
    description: impl IntoElement,
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
