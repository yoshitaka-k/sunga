use std::fs;
use std::path::{Path, PathBuf};
use getset::Getters;

use crate::{file, error};

/// 開いたファイルの管理
#[derive(Getters)]
#[getset(get = "pub")]
pub(crate) struct OpenFiles {
    images: Vec<file::Image>,
}

/// public methods
impl OpenFiles {
    /// 新しい OpenFiles を作成
    /// * `return` - OpenFiles のインスタンス
    pub fn new() -> Self {
        Self {
             images: Vec::new(),
        }
    }

    /// ファイルを追加
    /// * `file` - 追加するファイルのパス
    pub fn add(&mut self, path: PathBuf) -> error::Result<bool> {
        let base_dir = path.parent().unwrap_or(&path);

        if !self.find_file(&path, base_dir)? { return Ok(false); }

        Ok(true)
    }

    /// ファイルをクリア
    pub fn clear(&mut self) {
        self.images.clear();
    }
}

/// private methods
impl OpenFiles {
    /// ファイルを検索
    /// * `path` - ドロップされたファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `return` - ファイルのパス
    fn find_file(&mut self,
        path: &Path,
        base_dir: &Path
    ) -> error::Result<bool> {
        let metadata = path.metadata().map_err(|e| {
            error::ResizerError::file_error(e.to_string(), path.to_path_buf())
        })?;

        if metadata.is_file() {
            if file::is_allowed_extension(&path) {
                // strip_prefix は path を借りるので、
                // 先に String にして into_owned()で所有権を移す
                let relative_path = path.strip_prefix(base_dir)
                    .unwrap_or(&path)
                    .to_path_buf();

                // 画像ファイル構造体を作成
                let image = file::Image::new(path.to_path_buf(), relative_path);

                // 画像ファイル構造体を追加
                self.images.push(image);
            }
        } else if metadata.is_dir() {
            for entry in fs::read_dir(&path).map_err(|e| {
                error::ResizerError::file_error(e.to_string(), path.to_path_buf())
            })? {
                let entry = entry.map_err(|e| {
                    error::ResizerError::file_error(e.to_string(), path.to_path_buf())
                })?;

                // ディレクトリ内のファイルを検索
                self.find_file(&entry.path(), base_dir)?;
            }
        }

        Ok(true)
    }
}
