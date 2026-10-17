use std::path::PathBuf;

use eframe::egui::DroppedFileHandle;

use crate::{app, file, error};

/// ドロップされたファイルを処理
/// * `files` - ドロップされたファイル
pub(crate) fn files(
    files: &[DroppedFileHandle],
    app: &app::App,
    open_files: &mut file::OpenFiles,
) -> error::Result<bool> {
    if files.is_empty() {
        return Ok(false);
    }

    let mut result: bool = true;

    for file in files {
        if !path(app, file.path().to_path_buf(), open_files)? {
            result = false;
            break;
        }
    }

    Ok(result)
}

/// パスからファイルを開く
/// * `path` - ファイルのパス
/// * `open_file` - 開いているファイル
pub(crate) fn path(
    app: &app::App,
    path: PathBuf,
    open_files: &mut file::OpenFiles,
) -> error::Result<bool> {
    // ファイルを追加
    open_files.add(app, path)
}
