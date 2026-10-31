mod open_files;
mod image;
mod status;

pub(crate) use open_files::OpenFiles;
pub(crate) use image::Image;
pub(crate) use status::Status;

use std::path::Path;

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
