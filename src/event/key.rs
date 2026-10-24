use std::path::Path;
use crate::{error, file, resize};

pub(crate) fn up(
    id: u64,
    files: &mut file::OpenFiles
) -> error::Result<()> {
    let mut min = files.images().iter().map(|image| *image.id()).min().unwrap_or(1);
    if id > min { min = id - 1; }

    files.set_selected_id(Some(min));

    Ok(())
}

pub(crate) fn down(
    id: u64,
    files: &mut file::OpenFiles
) -> error::Result<()> {
    let mut max = files.images().iter().map(|image| *image.id()).max().unwrap_or(1);
    if id < max { max = id + 1; }

    files.set_selected_id(Some(max));

    Ok(())
}

/// バックスペースキーが押されたらファイルをキャンセルする
/// * `files` - ファイル一覧
/// * `resize_job` - リサイズジョブ
pub(crate) fn backspace(
    files: &mut file::OpenFiles,
    resize_job: &mut resize::ResizeJob
) -> error::Result<()> {
    let Some(image) = files.selected_image() else { return Ok(()) };
    let id = *image.id();

    // 選択中のファイルをキャンセル
    // リサイズ中のファイルはリサイズジョブにも中断を伝える
    // 待機中・リサイズ中以外のファイルはキャンセルできない
    if image.is_resizing() {
        resize_job.add_canceled_id(id)?;
    } else if !image.is_standby() {
        return Ok(());
    }

    files.set_status_canceled(id);

    Ok(())
}

/// スペースキーが押されたらファイルを選択表示する
/// * `path` - ファイルのパス
pub(crate) fn space(path: &Path) -> error::Result<()> {
    if !path.exists() {
        return Err(error::SungaError::file_not_found("File does not exist", path.to_path_buf()));
    }

    // ファイルをプレビュー表示
    preview_file_command(path)
}

/// QuickLook でファイルを表示する
#[cfg(target_os = "macos")]
fn preview_file_command(path: &Path) -> error::Result<()> {
    std::process::Command::new("qlmanage").arg("-p").arg(path).spawn()?;
    Ok(())
}

/// Windows では QuickLook を使用しない
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "windows")]
fn preview_file_command(_path: &Path) -> error::Result<()> {
    Ok(())
}

/// Linux では QuickLook を使用しない
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "linux")]
fn preview_file_command(_path: &Path) -> error::Result<()> {
    Ok(())
}
