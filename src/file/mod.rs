mod open_files;
mod image;
mod status;
mod token;
mod scan_job;
mod dimensions_job;

pub(crate) use open_files::OpenFiles;
pub(crate) use image::Image;
pub(crate) use status::Status;
pub(crate) use token::ScanToken;
pub(crate) use scan_job::ScanJob;
pub(crate) use dimensions_job::DimensionsJob;

use std::path::{Path, PathBuf};

/// ファイルスキャン結果
/// * `Found` - ファイルが見つかった
/// * `Failed` - ファイルが見つからなかった
pub(crate) enum ScanResult {
    Found { path: PathBuf, base_dir: PathBuf },
    Failed { path: PathBuf, base_dir: PathBuf, error: String },
}

/// 画像の寸法の状態
/// * `Pending` - まだ読んでいない
/// * `Loading` - 読んでいる最中
/// * `Ready` - 読めた
/// * `Unavailable` - 読めなかった
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Dimensions {
    Pending,
    Loading,
    Ready(u32, u32),
    Unavailable,
}

/// 読み込める拡張子
/// * `return` - 読み込める拡張子
pub(crate) fn allowed_extensions() -> Vec<&'static str> {
    vec!["png", "jpg", "jpeg"]
}

/// ファイルの拡張子が許可されているかどうかを確認
/// * `path` - ファイルのパス
/// * `return` - 許可されているかどうか
pub(crate) fn is_allowed_extension(path: &Path) -> bool {
    let extensions = allowed_extensions();
    if let Some(ext) = path.extension()
        && let Some(ext) = ext.to_str()
        && extensions.iter().any(|e| e.eq_ignore_ascii_case(ext))
    {
        return true;
    }
    false
}
