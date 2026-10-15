use std::fs;
use std::path::{Path, PathBuf};
use getset::Getters;

use crate::{app, file, error};

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
    pub fn add(&mut self, app: &app::App, path: PathBuf) -> error::Result<bool> {
        let base_dir = path.parent().unwrap_or(&path);

        if !self.find_file(app, &path, base_dir)? { return Ok(false); }

        Ok(true)
    }

    /// ファイルをクリア
    pub fn clear(&mut self) {
        self.images.clear();
    }

    /// ファイルが空かどうか
    /// * `return` - ファイルが空かどうか
    pub fn is_empty(&self) -> bool {
        self.images.is_empty()
    }

    /// 未処理ファイルがあるかどうか
    /// * `return` - 未処理ファイルがあるかどうか
    pub fn has_standby(&self) -> bool {
        self.images.iter().any(|f| f.is_standby())
    }

    /// 待機中のファイルを1件取得
    /// * `return` - 待機中のファイル1件
    pub fn get_standby_file(&mut self) -> Option<&mut file::Image> {
        self.images.iter_mut().find(|f| f.is_standby())
    }

    /// リサイズ結果を反映
    /// * `result` - リサイズ結果
    pub fn apply_result(&mut self, result: file::Image) {
        if let Some(image) = self.images.iter_mut().find(|f| f.id() == result.id()) {
            // リサイズ済みは反映しない
            if image.is_resized() { return; }

            // リサイズ結果を反映
            *image = result;
        }
    }
}

/// private methods
impl OpenFiles {
    /// ファイルが待機中かリサイズ中かどうかを確認
    /// * `path` - ファイルのパス
    /// * `return` - 待機中かリサイズ中かどうか
    fn is_standby_or_resizing(&self, path: &Path) -> bool {
        self.images.iter().any(|f| f.path() == path && f.is_standby_or_resizing())
    }

    /// ファイルがリサイズ済みかどうかを確認
    /// * `path` - ファイルのパス
    /// * `return` - リサイズ済みかどうか
    fn is_input_path(&self, path: &Path) -> bool {
        self.images.iter().any(|f| {
            f.path() == path
            && f.is_resized()
        })
    }

    /// ファイルが入力パスと出力パスが同じかどうかを確認
    /// * `path` - ファイルのパス
    /// * `output_path` - 出力パス
    /// * `return` - 入力パスと出力パスが同じかどうか
    fn is_input_and_output(&self, path: &Path, output_path: &Path) -> bool {
        self.images.iter().any(|f| {
            f.path() == path
                && f.output_path().as_deref() == Some(output_path)
                && f.is_resized()
        })
    }

    /// ファイルを検索
    /// * `path` - ドロップされたファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `return` - ファイルのパス
    fn find_file(&mut self,
        app: &app::App,
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
                let mut image = file::Image::new(path.to_path_buf(), relative_path);

                // 同じパスはスキップ
                let output_path = image.make_output_path(app);
                if *app.skip_same_path()
                  && self.is_input_and_output(&path, &output_path) {
                    image.set_status(file::Status::Skipped);
                    self.images.push(image);
                    return Ok(false);
                }

                // 同じパスが待機中かリサイズ中の場合はエラーを返す
                if self.is_standby_or_resizing(&path) {
                    image.set_status(file::Status::Error("Already in progress".to_string()));
                    self.images.push(image);
                    return Ok(false);
                }

                // 同じパスがリサイズ済みの場合はエラーを返す
                if self.is_input_path(&path) {
                    image.set_status(file::Status::Error("Already resized".to_string()));
                    self.images.push(image);
                    return Ok(false);
                }

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
                self.find_file(app, &entry.path(), base_dir)?;
            }
        }

        Ok(true)
    }
}
