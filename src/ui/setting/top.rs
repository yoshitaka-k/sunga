use crate::ui::{self, setting};
use crate::ui::assets::label;

/// 設定ウィンドウの上部パネルを描画
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    setting_token: &mut ui::SettingToken,
) {
    let style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);

    egui::Panel::top(setting::TOP_PANEL_NAME).frame(style).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut setting_token.tab, ui::SettingTab::General, label::Setting::general());
            ui.selectable_value(&mut setting_token.tab, ui::SettingTab::About, label::Setting::about());
        });
    });
}
