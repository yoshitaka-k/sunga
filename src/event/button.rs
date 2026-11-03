use crate::{error, file, resize, state};

/// 開いているファイルをクリア
/// * `files` - 開いているファイル
/// * `scan_job` - スキャンジョブ
/// * `resize_job` - リサイズジョブ
pub(crate) fn cancel_and_clear_files(
    files: &mut file::OpenFiles,
    scan_job: &file::ScanJob,
    resize_job: &mut resize::ResizeJob,
) -> error::Result<()> {
    // スキャンをキャンセル
    scan_job.stop_running();

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
    setting_token: &mut state::SettingToken,
    pos: Option<egui::Pos2>,
) {
    // タブを初期化
    setting_token.tab = state::SettingTab::General;

    // 設定ウィンドウの表示位置を設定
    setting_token.pos = pos;

    // 設定ウィンドウを開く
    setting_token.open = true;
}
