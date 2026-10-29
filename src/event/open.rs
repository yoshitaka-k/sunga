use std::path::PathBuf;
use crate::{config, file};

/// ファイルオープンダイアログを開いて選択結果を追加する
/// * `config` - 設定
/// * `label` - ファイルダイアログのラベル
/// * `open_files` - 開いているファイル
pub(crate) fn file(
    config: &config::Config,
    label: &str,
    open_files: &mut file::OpenFiles,
) {
    let extensions = file::allowed_extensions();

    // ファイルを選択
    let paths = rfd::FileDialog::new()
        .add_filter(label, &extensions)
        .pick_files();

    // ファイルを追加
    if let Some(paths) = paths {
        open_paths(config, open_files, paths);
    }
}

/// フォルダダイアログを開いて選択結果を追加する
/// * `config` - 設定
/// * `label` - フォルダダイアログのラベル
/// * `open_files` - 開いているファイル
pub(crate) fn folder(
    config: &config::Config,
    label: &str,
    open_files: &mut file::OpenFiles,
) {
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
    if let Some(paths) = paths {
        open_paths(config, open_files, paths);
    }
}

/// ドロップされたファイルを処理
/// * `ctx` - コンテキスト
/// * `config` - 設定
/// * `open_files` - 開いているファイル
/// * `files` - ドロップされたファイルのパス
pub(crate) fn drop_open(
    ctx: &egui::Context,
    config: &config::Config,
    open_files: &mut file::OpenFiles,
    files: Vec<PathBuf>,
) {
    open_paths(config, open_files, files);

    // ウィンドウを前面に持ってくる
    ctx.send_viewport_cmd_to(
        egui::ViewportId::ROOT,
        egui::ViewportCommand::Focus,
    );
}

/// ファイルを追加
/// * `config` - 設定
/// * `paths` - 追加するファイルのパス
/// * `open_files` - 開いているファイル
fn open_paths(
    config: &config::Config,
    open_files: &mut file::OpenFiles,
    paths: Vec<PathBuf>,
) {
    for path in paths {
        open_files.add(config, path);
    }
}
