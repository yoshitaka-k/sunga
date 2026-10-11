use std::path::PathBuf;
use crate::{error, file};

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `label` - ファイルダイアログのラベル
/// * `open_files` - 開いているファイル
/// * `return` - ファイルを開けたかどうか
pub(crate) fn file(
    label: &str,
    open_files: &mut file::OpenFiles,
) -> error::Result<bool> {
    let extensions = file::allowed_extensions();

    // ファイルを選択
    let paths = rfd::FileDialog::new()
        .add_filter(label, &extensions)
        .pick_files();

    // ファイルを追加
    open_paths(paths, open_files)
}

/// フォルダダイアログを開いて選択結果を追加する
/// * `label` - フォルダダイアログのラベル
/// * `open_files` - 開いているファイル
/// * `return` - フォルダを開けたかどうか
pub(crate) fn folder(
    label: &str,
    open_files: &mut file::OpenFiles,
) -> error::Result<bool> {
    let extensions = file::allowed_extensions();

    // フォルダを選択
    let paths = rfd::FileDialog::new()
        .add_filter(label, &extensions)
        .pick_folders();

    // ファイルを追加
    open_paths(paths, open_files)
}

/// ファイルを追加
/// * `path` - ファイルのパス
/// * `open_files` - 開いているファイル
/// * `return`: ファイルを開けたかどうか
fn open_paths(
    paths: Option<Vec<PathBuf>>,
    open_files: &mut file::OpenFiles,
) -> error::Result<bool> {
    if let Some(paths) = paths {
        for path in paths {
            if !open_files.add(path)? { return Ok(false); }
        }
    }

    Ok(true)
}
