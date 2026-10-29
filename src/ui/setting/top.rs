use crate::{state, lang};
use crate::ui::{self, setting};
use crate::ui::assets::{self, icon, svg};

/// 設定ウィンドウの上部パネルを描画
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    setting_token: &mut state::SettingToken,
) {
    let style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);
    let color = ui::icon_color(ui);

    // タブの選択時の背景色を保存
    let selection_bg_fill = ui.style_mut().visuals.selection.bg_fill;

    // タブの選択時の背景色を設定
    ui.style_mut().visuals.selection.bg_fill = assets::tab_selected_color(ui);

    egui::Panel::top(setting::TOP_PANEL_NAME).frame(style).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui::add_icon(ui, svg::SETTINGS, icon::SETTINGS_TOP_ICON_SIZE, color);

            ui.selectable_value(&mut setting_token.tab, state::SettingTab::General, lang::current().setting.general);
            ui.selectable_value(&mut setting_token.tab, state::SettingTab::Concurrent, lang::current().setting.concurrent);
            ui.selectable_value(&mut setting_token.tab, state::SettingTab::Resize, lang::current().setting.resize);
            ui.selectable_value(&mut setting_token.tab, state::SettingTab::About, lang::current().setting.about);
        });
    });

    // タブの選択時の背景色をリセット
    ui.style_mut().visuals.selection.bg_fill = selection_bg_fill;
}
