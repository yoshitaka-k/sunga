use crate::{app, error, file, resize, ui};
use crate::event::open;

/// ファイルダイアログを開く
/// * `app` - アプリケーション
/// * `files` - 開いているファイル
/// * `label` - ラベル
pub(crate) fn files_open(
    app: &app::App,
    files: &mut file::OpenFiles,
    label: &str,
) {
    open::file(app, label, files);
}

/// フォルダダイアログを開く
/// * `app` - アプリケーション
/// * `files` - 開いているファイル
/// * `label` - ラベル
pub(crate) fn folders_open(
    app: &app::App,
    files: &mut file::OpenFiles,
    label: &str,
) {
    open::folder(app, label, files);
}

/// 開いているファイルをクリア
/// * `ui` - UI
/// * `files` - 開いているファイル
pub(crate) fn cancel_and_clear_files(
    files: &mut file::OpenFiles,
    resize_job: &mut resize::ResizeJob,
) -> error::Result<()> {
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

    // エラーがある場合はエラーを返す
    if let Some(e) = error {
        return Err(e);
    }

    Ok(())
}

/// 設定ダイアログを開く
/// * `setting_token` - 設定ウィンドウトークン
/// * `pos` - 設定ウィンドウの表示位置
pub(crate) fn setting_open(
    setting_token: &mut ui::SettingToken,
    pos: Option<egui::Pos2>,
) {
    // タブを初期化
    setting_token.tab = ui::SettingTab::General;

    // 設定ウィンドウの表示位置を設定
    setting_token.pos = pos;

    // 設定ウィンドウを開く
    setting_token.open = true;
}
