use std::path::PathBuf;
use crate::{app, error, file};
use crate::event::drop;

/// ドロップされたファイルを処理する
/// * `ui` - ウィジェットのUI
/// * `app` - アプリケーション
/// * `open_files` - 開いているファイル
/// * `return` - ドロップがなければ `None`、あれば処理結果
pub(crate) fn drop(
    ui: &egui::Ui,
    app: &app::App,
    open_files: &mut file::OpenFiles,
) -> Option<error::Result<bool>> {
    ui.ctx().input(|input| {
        let files = input.raw.dropped_files.clone();
        if files.is_empty() {
            None
        } else {
            Some(drop::files(&files, app, open_files))
        }
    })
}

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `label` - ファイルダイアログのラベル
/// * `open_files` - 開いているファイル
/// * `return` - ファイルを開けたかどうか
pub(crate) fn file(
    app: &app::App,
    label: &str,
    open_files: &mut file::OpenFiles,
) -> error::Result<bool> {
    let extensions = file::allowed_extensions();

    // ファイルを選択
    let paths = rfd::FileDialog::new()
        .add_filter(label, &extensions)
        .pick_files();

    // ファイルを追加
    open_paths(app, paths, open_files)
}

/// フォルダダイアログを開いて選択結果を追加する
/// * `label` - フォルダダイアログのラベル
/// * `open_files` - 開いているファイル
/// * `return` - フォルダを開けたかどうか
pub(crate) fn folder(
    app: &app::App,
    label: &str,
    open_files: &mut file::OpenFiles,
) -> error::Result<bool> {
    let extensions = file::allowed_extensions();

    // Macのみファイルとフォルダを同時選択できる
    #[cfg(target_os = "macos")]
    let paths = rfd::FileDialog::new()
        .add_filter(label, &extensions)
        .pick_files_or_folders();

    // Mac以外は複数フォルダ選択のみ
    #[cfg(not(target_os = "macos"))]
    let paths = rfd::FileDialog::new()
        .add_filter(label, &extensions)
        .pick_folders();

    // ファイルを追加
    open_paths(app, paths, open_files)
}

/// ファイルを追加
/// * `path` - ファイルのパス
/// * `open_files` - 開いているファイル
/// * `return`: ファイルを開けたかどうか
fn open_paths(
    app: &app::App,
    paths: Option<Vec<PathBuf>>,
    open_files: &mut file::OpenFiles,
) -> error::Result<bool> {
    if let Some(paths) = paths {
        for path in paths {
            if !open_files.add(app, path)? { return Ok(false); }
        }
    }

    Ok(true)
}
