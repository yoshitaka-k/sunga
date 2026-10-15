use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering, AtomicBool};
use std::collections::HashSet;
use getset::{Getters, Setters};
use crate::{app, error, file, resize};

/// Image の一意 ID を発行する
const COUNT_UP: u64 = 1;
static NEXT_ID: AtomicU64 = AtomicU64::new(COUNT_UP);

/// 画像ファイル
#[derive(Clone, Getters, Setters)]
#[getset(get = "pub")]
pub(crate) struct Image {
    /// ファイルのID
    id: u64,

    /// ファイルのパス
    path: PathBuf,

    /// ファイルの相対パス
    relative_path: PathBuf,

    /// リサイズステータス
    #[getset(set = "pub")]
    status: file::Status,

    /// 出力先
    output_path: Option<PathBuf>,
}

/// public methods
impl Image {
    /// 新しい Image を作成
    /// * `path` - 画像ファイルのパス
    /// * `relative_path` - 画像ファイルの相対パス
    /// * `return` - Image のインスタンス
    pub fn new(path: PathBuf, relative_path: PathBuf) -> Self {
        // 一意 ID を発行
        let id = NEXT_ID.fetch_add(COUNT_UP, Ordering::Relaxed);

        Self {
            id,
            path,
            relative_path,
            status: file::Status::Standby,
            output_path: None,
        }
    }

    /// ファイルが待機中かどうか
    /// * `return` - ファイルが待機中かどうか
    pub fn is_standby(&self) -> bool {
        matches!(self.status, file::Status::Standby)
    }

    /// ファイルがリサイズ済みかどうか
    /// * `return` - ファイルがリサイズ済みかどうか
    pub fn is_resized(&self) -> bool {
        matches!(self.status, file::Status::Resized)
    }

    /// ファイルが待機中かリサイズ中かどうか
    /// * `return` - ファイルが待機中かリサイズ中かどうか
    pub fn is_standby_or_resizing(&self) -> bool {
        matches!(self.status, file::Status::Standby | file::Status::Resizing)
    }

    /// ファイル名を取得
    /// * `return` - ファイル名
    pub fn file_name(&self) -> String {
        self.path.file_name().unwrap_or_default().to_string_lossy().to_string()
    }
    /// 出力先を作成
    /// * `app` - アプリケーションのインスタンス
    /// * `return` - 出力先
    pub fn make_output_path(&self, app: &app::App) -> PathBuf {
        if app.output_path().is_empty() {
            self.path.clone()
        } else {
            PathBuf::from(app.output_path()).join(self.relative_path.clone())
        }
    }

    /// ファイルをリサイズ
    /// * `app` - アプリケーションのインスタンス
    /// * `running` - リサイズ実行中フラグ
    /// * `canceled` - リサイズキャンセルフラグ
    /// * `return` - リサイズステータス
    pub fn resize(
        &mut self,
        app: &app::App,
        running: Arc<AtomicBool>,
        canceled: Arc<Mutex<HashSet<u64>>>,
    ) -> error::Result<file::Status> {
        // 待機中・リサイズ中状態以外は実行しない
        if !self.is_standby_or_resizing() {
            return Ok(self.status.clone());
        }

        // 幅・高さが0の場合は実行しない
        if *app.width() == 0 && *app.height() == 0 {
            return Err(error::ResizerError::resize_error("Width or height is 0", self.path.clone()));
        }

        // 倍率が0.0の場合は実行しない
        if *app.scale() == 0.0 {
            return Err(error::ResizerError::resize_error("Scale is 0.0", self.path.clone()));
        }

        // 出力先
        let output_path = self.make_output_path(app);
        self.output_path = Some(output_path.clone());

        // リサイズトークン
        let token = resize::Token {
            id: self.id,
            running: running.clone(),
            canceled: canceled.clone(),
        };

        // ステータスをリサイズ中に設定
        self.status = file::Status::Resizing;

        // リサイズオプション
        let options = resize::Options {
            width: *app.width(),
            height: *app.height(),
            scale: *app.scale(),
        };

        match resize::Resizer::execute(&self.path, &output_path, &options, &token) {
            Ok(status) => {
                match status {
                    file::Status::Resized => {
                        self.status = file::Status::Resized;
                        Ok(self.status.clone())
                    }
                    file::Status::Canceled => {
                        self.status = file::Status::Canceled;
                        Ok(self.status.clone())
                    }
                    file::Status::Error(e) => {
                        self.status = file::Status::Error(e.clone());
                        Err(error::ResizerError::resize_error(e.to_string(), self.path.clone()))
                    }
                    _ => Ok(self.status.clone())
                }
            }
            Err(e) => {
                self.status = file::Status::Error(e.to_string());
                return Err(error::ResizerError::resize_error(e.to_string(), self.path.clone()));
            }
        }
    }
}

/// private methods
impl Image {
}
