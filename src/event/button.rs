use crate::{app, file, ui};

/// ファイルダイアログを開く
/// * `ui` - UI
/// * `dialog_token` - ファイルダイアログを開くトークン
pub(crate) fn files_open(
    ui: &egui::Ui,
    dialog_token: &mut ui::DialogToken,
) {
    dialog_token.file = true;

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// フォルダダイアログを開く
/// * `ui` - UI
/// * `dialog_token` - フォルダダイアログを開くトークン
pub(crate) fn folders_open(
    ui: &egui::Ui,
    dialog_token: &mut ui::DialogToken,
) {
    dialog_token.folder = true;

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// 開いているファイルをクリア
/// * `ui` - UI
/// * `files` - 開いているファイル
pub(crate) fn clear_files(
    ui: &egui::Ui,
    files: &mut file::OpenFiles,
) {
    files.clear();

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// 設定ダイアログを開く
/// * `ui` - UI
/// * `setting_token` - 設定ダイアログを開くトークン
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

/// アップデートを確認する
/// * `update_job` - 更新ジョブ
pub(crate) fn check_for_update(update_job: &mut app::UpdateJob) {
    update_job.run();
}
