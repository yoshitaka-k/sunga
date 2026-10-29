use std::fs;
use std::path::{Path, PathBuf};
use getset::{Getters, Setters};

use crate::{app, file};

/// ファイル情報
struct FileInfo {
    standby_len: u32,
    resizing_len: u32,
    resized_len: u32,
    unchanged_len: u32,
    skipped_len: u32,
    canceled_len: u32,
    error_len: u32,
    completed_len: u32,
    total_duration: u64,
    average_duration: u64,
}
impl FileInfo {
    pub fn new() -> Self {
        Self {
            standby_len: 0,
            resizing_len: 0,
            resized_len: 0,
            unchanged_len: 0,
            skipped_len: 0,
            canceled_len: 0,
            error_len: 0,
            completed_len: 0,
            total_duration: 0,
            average_duration: 0,
        }
    }
}

/// 開いたファイルの管理
#[derive(Getters, Setters)]
#[getset(get = "pub")]
pub(crate) struct OpenFiles {
    images: Vec<file::Image>,
    file_info: FileInfo,

    #[getset(set = "pub")]
    selected_id: Option<u64>,
}

/// public methods
impl OpenFiles {
    /// 新しい OpenFiles を作成
    /// * `return` - OpenFiles のインスタンス
    pub fn new() -> Self {
        Self {
             images: Vec::new(),
             file_info: FileInfo::new(),
             selected_id: None,
        }
    }

    /// ファイルを追加
    /// * `app` - アプリケーションのインスタンス
    /// * `path` - 追加するファイルのパス
    pub fn add(&mut self, app: &app::App, path: PathBuf) {
        let base_dir = path.parent().unwrap_or(&path);

        self.find_file(app, &path, base_dir);
    }

    /// 諸々クリア
    pub fn clear(&mut self) {
        self.images.clear();
        self.selected_id = None;
    }

    /// ファイル情報を更新
    pub fn update_file_info(&mut self) {
        let mut standby_len = 0;
        let mut resizing_len = 0;
        let mut resized_len = 0;
        let mut unchanged_len = 0;
        let mut skipped_len = 0;
        let mut canceled_len = 0;
        let mut error_len = 0;
        let mut total_duration = 0;

        // 各ファイルの情報を集計
        for file in &self.images {
            match file.status() {
                file::Status::Standby => standby_len += 1,
                file::Status::Resizing => resizing_len += 1,
                file::Status::Resized => {
                    resized_len += 1;
                    total_duration += *file.duration();
                }
                file::Status::Unchanged => unchanged_len += 1,
                file::Status::Skipped => skipped_len += 1,
                file::Status::Canceled => canceled_len += 1,
                file::Status::Error(_) => error_len += 1,
            }
        }

        // ファイル情報を更新
        self.file_info.standby_len = standby_len;
        self.file_info.resizing_len = resizing_len;
        self.file_info.resized_len = resized_len;
        self.file_info.unchanged_len = unchanged_len;
        self.file_info.skipped_len = skipped_len;
        self.file_info.canceled_len = canceled_len;
        self.file_info.error_len = error_len;
        self.file_info.completed_len = resized_len + unchanged_len + skipped_len;
        self.file_info.total_duration = total_duration;
        self.file_info.average_duration = if resized_len > 0 {
            total_duration / resized_len as u64
        } else { 0 };
    }

    /// ファイルの件数を取得
    /// * `return` - ファイルの件数
    pub fn len(&self) -> usize {
        self.images.len()
    }

    /// 待機中のファイルの数を取得
    /// * `return` - 待機中のファイルの数
    pub fn standby_len(&self) -> u32 {
        self.file_info.standby_len
    }

    /// リサイズ中のファイルの数を取得
    /// * `return` - リサイズ中のファイルの数
    pub fn resizing_len(&self) -> u32 {
        self.file_info.resizing_len
    }

    /// リサイズ済みのファイルの数を取得
    /// * `return` - リサイズ済みのファイルの数
    pub fn resized_len(&self) -> u32 {
        self.file_info.resized_len
    }

    /// 変更なしのファイルの数を取得
    /// * `return` - 変更なしのファイルの数
    pub fn unchanged_len(&self) -> u32 {
        self.file_info.unchanged_len
    }

    /// スキップされたファイルの数を取得
    /// * `return` - スキップされたファイルの数
    pub fn skipped_len(&self) -> u32 {
        self.file_info.skipped_len
    }

    /// エラーのファイルの数を取得
    /// * `return` - エラーのファイルの数
    pub fn error_len(&self) -> u32 {
        self.file_info.error_len
    }

    /// 完了したファイルの数を取得
    /// * `return` - 完了したファイルの数
    pub fn completed_len(&self) -> u32 {
        self.file_info.completed_len
    }

    /// リサイズ時間の平均を取得
    /// * `return` - リサイズ時間の平均
    pub fn average_duration(&self) -> u64 {
        self.file_info.average_duration
    }

    /// ジョブの状態を取得
    /// 優先度: リサイズ中 > エラー > リサイズ済み・変更なし・スキップ > 待機
    /// * `return` - ジョブの状態
    pub fn job_status(&self) -> file::Status {
        if self.file_info.resizing_len > 0 {
            file::Status::Resizing
        } else if self.file_info.error_len > 0 {
            file::Status::Error(String::new())
        } else if (self.file_info.resized_len + self.file_info.unchanged_len + self.file_info.skipped_len) > 0 {
            file::Status::Resized
        } else {
            file::Status::Standby
        }
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

    /// リサイズ中のファイルがあるかどうか
    /// * `return` - リサイズ中のファイルがあるかどうか
    pub fn has_resizing(&self) -> bool {
        self.images.iter().any(|f| f.is_resizing())
    }

    /// エラーのファイルがあるかどうか
    /// * `return` - エラーのファイルがあるかどうか
    pub fn has_error(&self) -> bool {
        self.images.iter().any(|f| f.is_error())
    }

    /// 待機中のファイルを1件取得
    /// * `return` - 待機中のファイル1件
    pub fn get_standby_file(&mut self) -> Option<&mut file::Image> {
        self.images.iter_mut().find(|f| f.is_standby())
    }

    /// 選択されたファイルのインデックスを取得
    /// * `return` - 選択されたファイルのインデックス
    pub fn selected_index(&self) -> Option<usize> {
        if let Some(id) = self.selected_id {
            self.images.iter().position(|f| *f.id() == id)
        } else {
            None
        }
    }

    /// 選択されたファイルを取得
    /// * `return` - 選択されたファイル
    pub fn selected_image(&self) -> Option<&file::Image> {
        if let Some(id) = self.selected_id {
            self.images.iter().find(|f| *f.id() == id)
        } else {
            None
        }
    }

    /// キャンセルされたファイルのステータスを更新
    /// 待機中とリサイズ中のファイルはキャンセルできる
    /// * `id` - キャンセルされたファイルの ID
    pub fn set_status_canceled(&mut self, id: u64) {
        if let Some(image) = self.images.iter_mut().find(|i| *i.id() == id)
            && image.is_standby_or_resizing()
        {
            image.set_status(file::Status::Canceled);
        }
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

    /// ファイルが入力パスと出力パスが同じかどうかを確認
    /// * `path` - ファイルのパス
    /// * `output_path` - 出力パス
    /// * `return` - 入力パスと出力パスが同じかどうか
    fn is_input_and_output(&self, path: &Path, output_path: &Path) -> bool {
        self.images.iter().any(|f| {
            f.path() == path
                && f.output_path().as_deref() == Some(output_path)
                && (f.is_resized() || f.is_unchanged())
        })
    }

    /// ファイルが存在しない場合の処理
    /// * `path` - ファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    /// * `error` - エラー内容
    fn error_file(
        &mut self,
        path: &Path,
        base_dir: &Path,
        error: String
    ) {
        let relative_path = path.strip_prefix(base_dir);
        let is_in_folder = relative_path.as_ref().is_ok_and(|p| p.components().count() > 1);
        let relative_path = relative_path.unwrap_or(path).to_path_buf();

        let mut image = file::Image::new(path.to_path_buf(), relative_path, is_in_folder);
        image.set_status(file::Status::Error(error));
        self.images.push(image);
    }

    /// ファイルを検索
    /// * `path` - ドロップされたファイルのパス
    /// * `base_dir` - ドロップされたパスの親（相対パスの基準）
    fn find_file(&mut self,
        app: &app::App,
        path: &Path,
        base_dir: &Path
    ) {
        // ファイルの存在を確認
        // シンボリックリンクは無視する
        let metadata = match path.symlink_metadata() {
            Ok(metadata) => metadata,
            Err(e) => return self.error_file(path, base_dir, e.to_string()),
        };

        if metadata.is_file() {
            if file::is_allowed_extension(path) {
                // base_dir からの相対パスを求める
                let relative_path = path.strip_prefix(base_dir);

                // フォルダ経由で追加されたファイルかどうか
                let is_in_folder = relative_path.as_ref().is_ok_and(|p| p.components().count() > 1);

                // 所有権のある PathBuf にする
                let relative_path = relative_path.unwrap_or(path).to_path_buf();

                // 画像ファイル構造体を作成
                let mut image = file::Image::new(path.to_path_buf(), relative_path, is_in_folder);

                // 出力パスを作成
                let output_path = image.make_output_path(app);

                // 同じパスが待機中かリサイズ中の場合はエラーを返す
                if self.is_standby_or_resizing(path) {
                    image.set_status(file::Status::Error("Already in progress".to_string()));
                    self.images.push(image);
                    return;
                }

                // 同じパスはスキップ
                if *app.skip_same_path() && self.is_input_and_output(path, &output_path) {
                    image.set_status(file::Status::Skipped);
                    self.images.push(image);
                    return;
                }

                // 画像ファイル構造体を追加
                self.images.push(image);
            }
        } else if metadata.is_dir() {
            let entries = match fs::read_dir(path) {
                Ok(entries) => entries,
                Err(e) => return self.error_file(path, base_dir, e.to_string()),
            };

            // ディレクトリ内のファイルを検索
            for entry in entries {
                match entry {
                    Ok(entry) => self.find_file(app, &entry.path(), base_dir),
                    Err(e) => self.error_file(path, base_dir, e.to_string()),
                }
            }
        }
    }
}
