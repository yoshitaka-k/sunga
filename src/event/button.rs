use crate::{app, error, file, resize, ui};

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
pub(crate) fn cancel_and_clear_files(
    ui: &egui::Ui,
    files: &mut file::OpenFiles,
    resize_job: &mut resize::ResizeJob,
) -> error::Result<()> {
    // リサイズをキャンセル
    resize_job.stop_running();

    // エラーを格納する変数
    let mut error = None;

    // キャンセルにファイル ID を追加
    for file in files.images() {
        // 待機中またはリサイズ中でない場合はスキップ
        if !file.is_standby_or_resizing() { continue; }

        if let Err(e) = resize_job.add_canceled_id(*file.id()) {
            error = Some(e);
            break;
        }
    }

    // 開いているファイルをクリア
    files.clear();

    // 再描画を要求
    ui.ctx().request_repaint();

    // エラーがある場合はエラーを返す
    if let Some(e) = error {
        return Err(e);
    }

    Ok(())
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
