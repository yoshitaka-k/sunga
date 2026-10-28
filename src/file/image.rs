use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering, AtomicBool};
use std::collections::HashSet;
use getset::{Getters, Setters};
use crate::{app, error, file, resize};

/// Image の一意 ID を発行する
const COUNT_UP: u64 = 1;
static NEXT_ID: AtomicU64 = AtomicU64::new(COUNT_UP);

/// リサイズ時間（ミリ秒）
const DEFAULT_DURATION: u64 = 0;

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

    /// 出力先
    output_path: Option<PathBuf>,

    /// フォルダ経由で追加されたファイルかどうか
    is_in_folder: bool,

    /// リサイズステータス
    #[getset(set = "pub")]
    status: file::Status,

    /// リサイズ時間（ミリ秒）
    #[getset(set = "pub")]
    duration: u64,
}

/// public methods
impl Image {
    /// 新しい Image を作成
    /// * `path` - 画像ファイルのパス
    /// * `relative_path` - 画像ファイルの相対パス
    /// * `is_in_folder` - フォルダ経由で追加されたファイルかどうか
    /// * `return` - Image のインスタンス
    pub fn new(path: PathBuf, relative_path: PathBuf, is_in_folder: bool) -> Self {
        // 一意 ID を発行
        let id = NEXT_ID.fetch_add(COUNT_UP, Ordering::Relaxed);

        Self {
            id,
            path,
            relative_path,
            status: file::Status::Standby,
            output_path: None,
            is_in_folder,
            duration: DEFAULT_DURATION,
        }
    }

    /// ファイルが待機中かどうか
    /// * `return` - ファイルが待機中かどうか
    pub fn is_standby(&self) -> bool {
        matches!(self.status, file::Status::Standby)
    }

    /// ファイルがリサイズ中かどうか
    /// * `return` - ファイルがリサイズ中かどうか
    pub fn is_resizing(&self) -> bool {
        matches!(self.status, file::Status::Resizing)
    }

    /// ファイルがリサイズ済みかどうか
    /// * `return` - ファイルがリサイズ済みかどうか
    pub fn is_resized(&self) -> bool {
        matches!(self.status, file::Status::Resized)
    }

    /// ファイルが変更なしの場合
    /// * `return` - ファイルが変更なしの場合
    pub fn is_unchanged(&self) -> bool {
        matches!(self.status, file::Status::Unchanged)
    }

    /// ファイルがエラーかどうか
    /// * `return` - ファイルがエラーかどうか
    pub fn is_error(&self) -> bool {
        matches!(self.status, file::Status::Error(_))
    }

    /// ファイルが待機中かリサイズ中かどうか
    /// * `return` - ファイルが待機中かリサイズ中かどうか
    pub fn is_standby_or_resizing(&self) -> bool {
        matches!(self.status, file::Status::Standby | file::Status::Resizing)
    }

    /// ファイル名を取得
    /// * `return` - ファイル名
    pub fn file_name(&self) -> std::borrow::Cow<'_, str> {
        self.path.file_name().map(|f| f.to_string_lossy()).unwrap_or_default()
    }

    /// 出力ファイルのパスを取得
    /// * `return` - 出力ファイルのパス
    pub fn reveal_path(&self) -> &PathBuf {
        match self.output_path {
            Some(ref path) if path.exists() => path,
            _ => &self.path,
        }
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

        // 範囲外の場合はエラーメッセージを設定
        let out_of_range = if !(resize::SIZED_RANGE_MIN..=resize::SIZED_RANGE_MAX).contains(app.width()) {
            // 幅が範囲外の場合は実行しない
            Some(format!("Width is out of range ({}..{})", resize::SIZED_RANGE_MIN, resize::SIZED_RANGE_MAX))
        } else  if !(resize::SIZED_RANGE_MIN..=resize::SIZED_RANGE_MAX).contains(app.height()) {
            // 高さが範囲外の場合は実行しない
            Some(format!("Height is out of range ({}..{})", resize::SIZED_RANGE_MIN, resize::SIZED_RANGE_MAX))
        } else  if !(resize::SCALE_RANGE_MIN..=resize::SCALE_RANGE_MAX).contains(app.scale()) {
            // 倍率が範囲外の場合は実行しない
            Some(format!("Scale is out of range ({}..{})", resize::SCALE_RANGE_MIN, resize::SCALE_RANGE_MAX))
        } else {
            None
        };

        // 範囲外の場合はエラーメッセージを設定
        if let Some(msg) = out_of_range {
            self.status = file::Status::Error(msg.clone());
            return Err(error::SungaError::resize_error(msg, self.path.clone()));
        }

        // 処理開始時間を取得
        let start_time = std::time::Instant::now();

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
            is_scale: *app.is_scale(),
        };

        match resize::Sunga::execute(&self.path, &output_path, &options, &token) {
            Ok(status) => {
                match status {
                    file::Status::Resized => {
                        // 処理終了時間を取得
                        let end_time = std::time::Instant::now();
                        // 処理時間を計算
                        let duration = end_time.duration_since(start_time).as_millis();
                        self.duration = duration as u64;

                        // ステータスをリサイズ済みに設定
                        self.status = file::Status::Resized;
                        Ok(self.status.clone())
                    }
                    file::Status::Unchanged => {
                        self.status = file::Status::Unchanged;
                        Ok(self.status.clone())
                    }
                    file::Status::Skipped => {
                        self.status = file::Status::Skipped;
                        Ok(self.status.clone())
                    }
                    file::Status::Canceled => {
                        self.status = file::Status::Canceled;
                        Ok(self.status.clone())
                    }
                    file::Status::Error(e) => {
                        self.status = file::Status::Error(e.clone());
                        Err(error::SungaError::resize_error(e.to_string(), self.path.clone()))
                    }
                    _ => Ok(self.status.clone())
                }
            }
            Err(e) => {
                self.status = file::Status::Error(e.to_string());
                return Err(error::SungaError::resize_error(e.to_string(), self.path.clone()));
            }
        }
    }
}

/// private methods
impl Image {
}
