use crate::ui::{self, setting};
use crate::ui::assets::{icon, label, svg};

/// 設定ウィンドウの上部パネルを描画
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    setting_token: &mut ui::SettingToken,
) {
    let style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);
    let color = ui::icon_color(ui);

    egui::Panel::top(setting::TOP_PANEL_NAME).frame(style).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui::add_icon(ui, svg::SETTINGS, icon::SETTINGS_TOP_ICON_SIZE, color);

            ui.selectable_value(&mut setting_token.tab, ui::SettingTab::General, label::current().setting.general);
            ui.selectable_value(&mut setting_token.tab, ui::SettingTab::Concurrent, label::current().setting.concurrent);
            ui.selectable_value(&mut setting_token.tab, ui::SettingTab::Resize, label::current().setting.resize);
            ui.selectable_value(&mut setting_token.tab, ui::SettingTab::About, label::current().setting.about);
        });
    });
}
