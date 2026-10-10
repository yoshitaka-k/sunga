use crate::ui;

/// Command + W キーが押されたら設定ダイアログを閉じる
/// * `ui` - UI
/// * `setting_token` - 設定ダイアログのトークン
pub(crate) fn command_w(
    ui: &mut egui::Ui,
    setting_token: &mut ui::SettingToken,
) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::W)
    }) {
        setting_token.open = false;
    }
}
