mod job;
mod option;
pub(crate) use job::ResizeJob;
pub(crate) use option::Options;

use std::path::Path;
use std::sync::Arc;
use std::sync:: Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashSet;
use image::{self, GenericImageView};
use crate::{error, file, resize};

/// 一時ファイルの拡張子
const TEMP_EXTENSION: &str = "sunga.temp";

/// サイズの範囲
pub(crate) const SIZED_RANGE_MIN: u32 = 10;
pub(crate) const SIZED_RANGE_MAX: u32 = 10000;

/// 倍率の範囲
pub(crate) const SCALE_RANGE_MIN: u16 = 10;
pub(crate) const SCALE_RANGE_MAX: u16 = 500;

/// リサイズトークン
#[derive(Clone)]
pub struct Token {
    pub id: u64,
    pub running: Arc<AtomicBool>,
    pub canceled: Arc<Mutex<HashSet<u64>>>,
}

impl Token {
    /// キャンセルされたかどうか
    /// * `return` - キャンセルされたかどうか
    pub fn is_canceled(&self) -> error::Result<bool> {
        let canceled = self.canceled.lock().map_err(|_| {
            error::SungaError::lock_poisoned()
        })?;

        // リサイズ実行外またはキャンセルされた場合はキャンセルされたと判定
        Ok(!self.running.load(Ordering::Acquire) || canceled.contains(&self.id))
    }
}

/// リサイズ
pub struct Sunga;
/// public methods
impl Sunga {
    /// リサイズ
    /// * `path` - リサイズするファイルのパス
    /// * `output_path` - リサイズしたファイルの出力先のパス
    /// * `options` - リサイズオプション
    /// * `token` - リサイズトークン
    /// * `return` - リサイズが成功したかどうか
    pub fn execute(
        path: &Path,
        output_path: &Path,
        options: &resize::Options,
        token: &resize::Token,
    ) -> error::Result<file::Status> {
        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        // 画像を読み込む
        let reader = image::ImageReader::open(path)
            .and_then(|r| r.with_guessed_format())
            .map_err(|e| error::SungaError::resize_error(e.to_string(), path.to_path_buf()))?;

        // 画像のフォーマットを取得
        let format = reader.format().ok_or_else(|| {
            error::SungaError::resize_error("Unknown image format".to_string(), path.to_path_buf())
        })?;

        // フォーマットの拡張子を取得
        let extension = format.extensions_str()[0].to_string();

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        // 画像をデコード
        let image = reader.decode().map_err(|e| {
            error::SungaError::resize_error(e.to_string(), path.to_path_buf())
        })?;

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        // リサイズを実行
        let resized = Self::resize(&image, options);

        // 同じサイズの場合はスキップ
        if resized.width() == image.width() && resized.height() == image.height() {
            // 出力ファイルが存在しない場合は作成
            if path != output_path && !output_path.exists() {
                // 出力パスを作成
                Self::make_output_path(output_path)?;

                // ファイルをコピー
                std::fs::copy(path, output_path).map_err(|e| {
                    error::SungaError::resize_error(e.to_string(), output_path.to_path_buf())
                })?;
            }
            return Ok(file::Status::Unchanged);
        }

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        // 出力パスを作成
        Self::make_output_path(output_path)?;

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        // 一時ファイルのパスを作成
        let temp_path = output_path.with_extension(format!("{}.{}", extension, TEMP_EXTENSION));

        // リサイズ後の画像を保存
        let result = resized.save_with_format(&temp_path, format);

        // 保存に失敗した場合は一時ファイルを削除して終了
        if let Err(e) = result {
            // 一時ファイルを削除して終了
            let _ = Self::remove_file(&temp_path);
            return Err(error::SungaError::resize_error(e.to_string(), output_path.to_path_buf()));
        }

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? {
            // 一時ファイルを削除して終了
            let _ = Self::remove_file(&temp_path);
            return Ok(file::Status::Canceled);
        }

        // 一時ファイルを元のファイルに上書き
        let result = Self::replace_file(&temp_path, output_path);

        // 上書きに失敗した場合は一時ファイルを削除して終了
        if let Err(e) = result {
            // 一時ファイルを削除して終了
            let _ = Self::remove_file(&temp_path);
            return Err(e);
        }

        Ok(file::Status::Resized)
    }
}

/// private methods
impl Sunga {
    /// リサイズ
    /// * `image` - リサイズする画像
    /// * `options` - リサイズオプション
    /// * `return` - リサイズした画像
    fn resize(image: &image::DynamicImage, options: &resize::Options) -> image::DynamicImage {
        let (orig_width, orig_height) = image.dimensions();

        let (width, height) = if options.is_scale {
            // 倍率を使用する場合
            let scale = options.scale as f32 / 100.0;
            (
                (orig_width as f32 * scale) as u32,
                (orig_height as f32 * scale) as u32,
            )
        } else {
            // 幅と高さを使用する場合
            (options.width, options.height)
        };

        image.resize(width, height, image::imageops::FilterType::Lanczos3)
    }

    /// 出力パスを作成
    /// * `output_path` - 出力パス
    /// * `return` - 出力パス
    fn make_output_path(output_path: &Path) -> error::Result<()> {
        // 親ディレクトリを取得
        let Some(parent) = output_path.parent() else {
            return Err(error::SungaError::resize_error("Output path parent not found", output_path.to_path_buf()));
        };

        // 親ディレクトリを作成
        std::fs::create_dir_all(parent).map_err(|e| {
            error::SungaError::resize_error(e.to_string(), output_path.to_path_buf())
        })?;

        Ok(())
    }

    /// 一時ファイルを削除
    /// * `path` - 一時ファイルのパス
    /// * `return` - 一時ファイルを削除したかどうか
    fn remove_file(path: &Path) -> error::Result<()> {
        std::fs::remove_file(path).map_err(|e| {
            error::SungaError::resize_error(e.to_string(), path.to_path_buf())
        })
    }

    /// 一時ファイルを元のファイルに上書き
    /// * `from` - 一時ファイルのパス
    /// * `to` - 元のファイルのパス
    /// * `return` - 一時ファイルを元のファイルに上書きしたかどうか
    fn replace_file(from: &Path, to: &Path) -> error::Result<()> {
        // Windows 以外の環境ではファイルを直接上書き
        #[cfg(not(target_os = "windows"))]
        {
            std::fs::rename(from, to).map_err(|e| {
                error::SungaError::resize_error(e.to_string(), to.to_path_buf())
            })
        }

        // Windows 環境では MoveFileExW を使用してファイルを上書き
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::ffi::OsStrExt;
            use winapi::um::winbase::{MoveFileExW, MOVEFILE_WRITE_THROUGH, MOVEFILE_REPLACE_EXISTING};

            // Windows の API に渡すためにパスを UTF-16 に変換
            let from_win: Vec<u16> = from.as_os_str().encode_wide().chain(Some(0)).collect();
            let to_win: Vec<u16> = to.as_os_str().encode_wide().chain(Some(0)).collect();

            // MoveFileExW を呼び出してファイルを上書き
            let result = unsafe {
                MoveFileExW(from_win.as_ptr(), to_win.as_ptr(), MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH)
            };

            // エラーが発生した場合はエラーを返す
            if result == 0 {
                return Err(error::SungaError::resize_error(std::io::Error::last_os_error().to_string(), to.to_path_buf()));
            }

            Ok(())
        }
    }
}
