use crate::{error, file, ui};
use crate::event::drop;

/// ドロップされたファイルを処理する
/// * `ui` - ウィジェットのUI
/// * `open_files` - 開いているファイル
/// * `return` - ドロップがなければ `None`、あれば処理結果
pub(crate) fn drop(
    ui: &egui::Ui,
    open_files: &mut file::OpenFiles,
) -> Option<error::Result<bool>> {
    ui.ctx().input(|input| {
        let files = input.raw.dropped_files.clone();
        if files.is_empty() {
            None
        } else {
            Some(drop::files(&files, open_files))
        }
    })
}

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
