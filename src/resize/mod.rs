mod job;
mod token;
mod option;
mod guard;
pub(crate) use job::ResizeJob;
pub(crate) use token::Token;
pub(crate) use option::Options;

use std::path::Path;
use std::io::BufWriter;
use image::{self, GenericImageView, ImageDecoder, DynamicImage, ImageEncoder};
use image::codecs::{jpeg::JpegEncoder, png::PngEncoder};
use crate::{error, file, resize};

/// 一時ファイルの拡張子
const TEMP_EXTENSION: &str = "sunga.temp";

/// JPEG の品質
const JPEG_QUALITY: u8 = 100;

/// サイズの範囲
pub(crate) const SIZED_RANGE_MIN: u32 = 10;
pub(crate) const SIZED_RANGE_MAX: u32 = 10000;

/// 倍率の範囲
pub(crate) const SCALE_RANGE_MIN: u16 = 10;
pub(crate) const SCALE_RANGE_MAX: u16 = 500;

/// リサイズ
pub struct Resize;
/// public methods
impl Resize {
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

        // リーダーをデコーダーに変換
        let mut decoder = reader.into_decoder().map_err(|e| {
            error::SungaError::resize_error(e.to_string(), path.to_path_buf())
        })?;

        // 情報を取得
        let orientation = decoder.orientation().map_err(|e| {
            error::SungaError::resize_error(e.to_string(), path.to_path_buf())
        })?;
        let exif = decoder.exif_metadata().map_err(|e| {
            error::SungaError::resize_error(e.to_string(), path.to_path_buf())
        })?;
        let icc = decoder.icc_profile().map_err(|e| {
            error::SungaError::resize_error(e.to_string(), path.to_path_buf())
        })?;

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        // 画像をデコードからピクセルデータへ変換
        let mut image = DynamicImage::from_decoder(decoder).map_err(|e| {
            error::SungaError::resize_error(e.to_string(), path.to_path_buf())
        })?;

        // 画像の向きを適用
        image.apply_orientation(orientation);

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
        // 一時ファイルのパスを作成
        let temp_path = output_path.with_extension(format!("{}.{}", extension, TEMP_EXTENSION));

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        // 一時ファイルを削除するためのガードを作成
        let mut guard = guard::ResizeGuard::new(temp_path.clone());

        // バッファを作成
        let mut buffer = Self::create_buffer(&temp_path)?;

        // リサイズ後の画像をエンコード
        match format {
            image::ImageFormat::Jpeg => {
                Self::write(&resized, JpegEncoder::new_with_quality(&mut buffer, JPEG_QUALITY), exif, icc, &temp_path)
            }
            image::ImageFormat::Png => {
                Self::write(&resized, PngEncoder::new(&mut buffer), exif, icc, &temp_path)
            }
            _ => return Err(error::SungaError::resize_error("Unsupported image format".to_string(), temp_path.to_path_buf())),
        }?;

        // バッファをクローズ
        buffer.into_inner().map_err(|e| {
            error::SungaError::resize_error(e.to_string(), temp_path.to_path_buf())
        })?;

        // 一時ファイルを元のファイルに上書き
        Self::replace_file(&temp_path, output_path)?;

        // 削除しない
        guard.keep();

        Ok(file::Status::Resized)
    }
}

/// private methods
impl Resize {
    /// リサイズ
    /// * `image` - リサイズする画像
    /// * `options` - リサイズオプション
    /// * `return` - リサイズした画像
    fn resize(image: &DynamicImage, options: &resize::Options) -> DynamicImage {
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
        })
    }

    /// バッファを作成
    /// * `path` - バッファのパス
    /// * `return` - バッファ
    fn create_buffer(path: &Path) -> error::Result<BufWriter<std::fs::File>> {
        let file = std::fs::File::create(path).map_err(|e| {
            error::SungaError::resize_error(e.to_string(), path.to_path_buf())
        })?;
        Ok(BufWriter::new(file))
    }

    /// 画像をエンコード
    /// * `image` - 画像
    /// * `encoder` - エンコーダー
    /// * `exif` - EXIF 情報
    /// * `icc` - ICC プロファイル
    /// * `return` - 画像をエンコードしたかどうか
    fn write<E: ImageEncoder>(
        image: &DynamicImage,
        mut encoder: E,
        exif: Option<Vec<u8>>,
        icc: Option<Vec<u8>>,
        path: &Path,
    ) -> error::Result<()> {
        // EXIF 情報を削除
        if let Some(mut exif) = exif {
            let _ = image::metadata::Orientation::remove_from_exif_chunk(&mut exif);
            let _ = encoder.set_exif_metadata(exif);
        }
        // ICC プロファイルを設定
        if let Some(icc) = icc {
            let _ = encoder.set_icc_profile(icc);
        }

        image.write_with_encoder(encoder).map_err(|e| {
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
