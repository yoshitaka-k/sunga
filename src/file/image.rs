use std::path::PathBuf;
use getset::Getters;

/// 画像ファイル
#[derive(Getters)]
#[getset(get = "pub")]
pub(crate) struct Image {
    /// ファイルのパス
    path: PathBuf,

    /// ファイルの相対パス
    relative_path: PathBuf,
}

/// public methods
impl Image {
    /// 新しい Image を作成
    /// * `path` - 画像ファイルのパス
    /// * `relative_path` - 画像ファイルの相対パス
    /// * `return` - Image のインスタンス
    pub fn new(path: PathBuf, relative_path: PathBuf) -> Self {
        Self { path, relative_path }
    }
}
