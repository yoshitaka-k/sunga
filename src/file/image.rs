use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use getset::{Getters, Setters};
use crate::file;

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
    #[getset(set = "pub")]
    output_path: Option<PathBuf>,

    /// 画像の寸法
    #[getset(set = "pub")]
    dimensions: file::Dimensions,

    /// 新しい画像の寸法
    resized_dimensions: Option<(u32, u32)>,

    /// フォルダ経由で追加されたファイルかどうか
    is_in_folder: bool,

    /// リサイズステータス
    #[getset(set = "pub")]
    status: file::Status,

    /// リサイズ時間（ミリ秒）
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
        Self {
            id: NEXT_ID.fetch_add(COUNT_UP, Ordering::Relaxed),
            path,
            relative_path,
            status: file::Status::Standby,
            output_path: None,
            dimensions: file::Dimensions::Pending,
            resized_dimensions: None,
            is_in_folder,
            duration: DEFAULT_DURATION,
        }
    }

    /// ファイルが寸法を読み込んでないかどうか
    /// * `return` - ファイルが寸法を読み込んでないかどうか
    pub fn is_pending(&self) -> bool {
        matches!(self.dimensions, file::Dimensions::Pending)
    }

    /// ファイルが寸法を読み込んでないかリサイズ中かどうか
    /// * `return` - ファイルが寸法を読み込んでないかリサイズ中かどうか
    pub fn is_pending_or_loading(&self) -> bool {
        matches!(self.dimensions, file::Dimensions::Pending | file::Dimensions::Loading)
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
    /// * `output_path` - 出力先
    /// * `return` - 出力先
    pub fn make_output_path(&self, output_path: &str) -> PathBuf {
        if output_path.is_empty() {
            self.path.clone()
        } else {
            PathBuf::from(output_path).join(self.relative_path.clone())
        }
    }

    /// リサイズ結果を設定
    /// * `width` - リサイズ後の幅
    /// * `height` - リサイズ後の高さ
    /// * `duration` - リサイズ時間（ミリ秒）
    pub fn set_resized(&mut self, (width, height): (u32, u32), duration: u64) {
        self.resized_dimensions = Some((width, height));
        self.duration = duration;
    }
}

/// 画像の寸法を取得
/// * `path` - 画像ファイルのパス
/// * `return` - 画像の寸法
pub fn open_dimensions(path: &PathBuf) -> Option<(u32, u32)> {
    image::ImageReader::open(path).ok()
        .and_then(|r| r.with_guessed_format().ok())
        .and_then(|r| r.into_dimensions().ok())
}
