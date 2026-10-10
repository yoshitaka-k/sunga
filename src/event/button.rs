use crate::ui;

// 設定ダイアログを開く
/// * `ui` - UI
/// * `setting_token` - 設定ダイアログのトークン
pub(crate) fn setting_open(
    ui: &egui::Ui,
    setting_token: &mut ui::SettingToken,
) {
    // 設定ダイアログを開く
    setting_token.open = true;

    // 設定ダイアログの表示位置を設定
    setting_token.pos = ui.ctx().input(|input| {
        input.viewport().outer_rect.map(|rect| rect.min)
    });

    // 再描画を要求
    ui.ctx().request_repaint();
}
