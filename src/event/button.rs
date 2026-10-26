use crate::{app, error, file, resize, ui};
use crate::event::open;

/// ファイルダイアログを開く
/// * `app` - アプリケーション
/// * `files` - 開いているファイル
/// * `label` - ラベル
/// * `return` - ファイルが開かれたかどうか
pub(crate) fn files_open(
    app: &app::App,
    files: &mut file::OpenFiles,
    label: &str,
) -> error::Result<bool> {
    open::file(app, label, files)
}

/// フォルダダイアログを開く
/// * `app` - アプリケーション
/// * `files` - 開いているファイル
/// * `label` - ラベル
/// * `return` - フォルダが開かれたかどうか
pub(crate) fn folders_open(
    app: &app::App,
    files: &mut file::OpenFiles,
    label: &str,
) -> error::Result<bool> {
    open::folder(app, label, files)
}

/// 開いているファイルをクリア
/// * `ui` - UI
/// * `files` - 開いているファイル
pub(crate) fn cancel_and_clear_files(
    ui: &egui::Ui,
    files: &mut file::OpenFiles,
    resize_job: &mut resize::ResizeJob,
) -> error::Result<bool> {
    // リサイズをキャンセル
    resize_job.stop_running();

    // エラーを格納する変数
    let mut error = None;

    // キャンセルにファイル ID を追加
    for image in files.images() {
        // リサイズ中でない場合はスキップ
        if !image.is_resizing() { continue; }

        if let Err(e) = resize_job.add_canceled_id(*image.id()) {
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

    Ok(true)
}

/// 設定ダイアログを開く
/// * `ui` - UI
/// * `setting_token` - 設定ダイアログを開くトークン
pub(crate) fn setting_open(
    ui: &egui::Ui,
    setting_token: &mut ui::SettingToken,
) -> error::Result<bool> {
    // 設定ダイアログを開く
    setting_token.open = true;

    // 設定ダイアログの表示位置を設定
    setting_token.pos = ui.ctx().input(|input| {
        input.viewport().outer_rect.map(|rect| rect.min)
    });

    // 再描画を要求
    ui.ctx().request_repaint();

    Ok(true)
}

/// アップデートを確認する
/// * `update_job` - 更新ジョブ
pub(crate) fn check_for_update(
    update_job: &mut app::UpdateJob
) -> error::Result<()> {
    update_job.run();

    Ok(())
}
