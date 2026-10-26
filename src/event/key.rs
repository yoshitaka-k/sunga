use std::path::Path;
use crate::{error, file, resize, ui};

/// 上矢印キーが押されたら選択された行を上に移動する
/// * `id` - 選択された行のID
/// * `files` - ファイル一覧
/// * `scroll_token` - スクロールエリアを表示するためのトークン
pub(crate) fn up(
    id: u64,
    files: &mut file::OpenFiles,
    scroll_token: &mut ui::ScrollToken
) -> error::Result<bool> {
    let mut min = files.images().iter().map(|image| *image.id()).min().unwrap_or(1);
    if id > min { min = id - 1; }

    files.set_selected_id(Some(min));
    scroll_token.selected = true;

    Ok(true)
}

/// 下矢印キーが押されたら選択された行を下に移動する
/// * `id` - 選択された行のID
/// * `files` - ファイル一覧
/// * `scroll_token` - スクロールエリアを表示するためのトークン
pub(crate) fn down(
    id: u64,
    files: &mut file::OpenFiles,
    scroll_token: &mut ui::ScrollToken
) -> error::Result<bool> {
    let mut max = files.images().iter().map(|image| *image.id()).max().unwrap_or(1);
    if id < max { max = id + 1; }

    files.set_selected_id(Some(max));
    scroll_token.selected = true;

    Ok(true)
}

/// バックスペースキーが押されたらファイルをキャンセルする
/// * `files` - ファイル一覧
/// * `resize_job` - リサイズジョブ
pub(crate) fn backspace(
    files: &mut file::OpenFiles,
    resize_job: &mut resize::ResizeJob
) -> error::Result<bool> {
    let Some(image) = files.selected_image() else { return Ok(false) };
    let id = *image.id();

    // 選択中のファイルをキャンセル
    // リサイズ中のファイルはリサイズジョブにも中断を伝える
    // 待機中・リサイズ中以外のファイルはキャンセルできない
    if image.is_resizing() {
        resize_job.add_canceled_id(id)?;
    } else if !image.is_standby() {
        return Ok(false);
    }

    files.set_status_canceled(id);

    Ok(true)
}

/// スペースキーが押されたらファイルを選択表示する
/// * `path` - ファイルのパス
pub(crate) fn space(path: &Path) -> error::Result<bool> {
    if !path.exists() {
        return Err(error::SungaError::file_not_found("File does not exist", path.to_path_buf()));
    }

    // ファイルをプレビュー表示
    preview_file_command(path)
}

/// QuickLook でファイルを表示する
#[cfg(target_os = "macos")]
fn preview_file_command(path: &Path) -> error::Result<bool> {
    std::process::Command::new("qlmanage").arg("-p").arg(path).spawn()?;
    Ok(true)
}

/// Windows では QuickLook を使用しない
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "windows")]
fn preview_file_command(_path: &Path) -> error::Result<bool> {
    Ok(true)
}

/// Linux では QuickLook を使用しない
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "linux")]
fn preview_file_command(_path: &Path) -> error::Result<bool> {
    Ok(true)
}
