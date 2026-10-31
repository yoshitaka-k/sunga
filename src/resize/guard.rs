use std::path::PathBuf;

/// リサイズガード
pub(crate) struct ResizeGuard {
    keep: bool,
    path: PathBuf,
}

impl ResizeGuard {
    /// 新しいリサイズガードを作成
    /// * `path` - 一時ファイルのパス
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self {
            keep: false,
            path,
        }
    }

    /// 削除しない
    pub fn keep(&mut self) {
        self.keep = true;
    }
}

/// Drop trait impl
impl Drop for ResizeGuard {
    /// 一時ファイルを削除
    fn drop(&mut self) {
        if !self.keep {
            // 一時ファイルを削除
            let _ = std::fs::remove_file(&self.path);
        }
    }
}
