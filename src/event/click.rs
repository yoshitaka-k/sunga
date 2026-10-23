use std::path::Path;
use crate::{error, file};

/// ファイルを選択状態にする
/// * `id` - ファイルのID
/// * `files` - ファイルのリスト
/// * `return` - エラーが発生したかどうか
pub(crate) fn click(
    id: u64,
    files: &mut file::OpenFiles
) -> error::Result<()> {
    files.set_selected_id(Some(id));
    Ok(())
}

/// ダブルクリックで Finder でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
pub(crate) fn double_click(path: &Path) -> error::Result<()> {
    if !path.exists() {
        return Err(error::SungaError::file_not_found("File does not exist", path.to_path_buf()));
    }

    // ファイルを選択表示
    reveal_file_command(path)
}

/// Finder でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "macos")]
fn reveal_file_command(path: &Path) -> error::Result<()> {
    std::process::Command::new("open").arg("-R").arg(path).status()?;
    Ok(())
}

/// Explorer でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "windows")]
fn reveal_file_command(path: &Path) -> error::Result<()> {
    std::process::Command::new("explorer").arg(format!("/select,{}", path.display())).status()?;
    Ok(())
}

/// XDG でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "linux")]
fn reveal_file_command(path: &Path) -> error::Result<()> {
    std::process::Command::new("xdg-open").arg("--reveal").arg(path).status()?;
    Ok(())
}
