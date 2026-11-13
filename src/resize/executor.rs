use std::path::Path;
use std::io::BufWriter;
use image::{self, GenericImageView, ImageDecoder, DynamicImage, ImageEncoder};
use image::codecs::{jpeg::JpegEncoder, png::PngEncoder};
use crate::{error, resize};
use crate::resize::{TEMP_EXTENSION, JPEG_QUALITY, ResizeGuard};

/// リサイズ
pub struct Resize;
/// public methods
impl Resize {
    /// リサイズ
    /// * `path` - リサイズするファイルのパス
    /// * `output_path` - リサイズしたファイルの出力先のパス
    /// * `options` - リサイズオプション
    /// * `token` - リサイズトークン
    /// * `return` - リサイズ結果
    pub fn execute(
        path: &Path,
        output_path: &Path,
        options: &resize::Options,
        token: &resize::Token,
    ) -> error::Result<resize::ExecuteResult> {
        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(resize::ExecuteResult::Canceled); }

        // 画像を読み込む
        let reader = image::ImageReader::open(path)
            .and_then(|r| r.with_guessed_format())
            .map_err(|e| error::SungaError::resize_error(e.to_string(), path.to_path_buf()))?;

        // 画像のフォーマットを取得
        let format = reader.format().ok_or_else(|| {
            error::SungaError::resize_error("Unknown image format".to_string(), path.to_path_buf())
        })?;

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
        if token.is_canceled()? { return Ok(resize::ExecuteResult::Canceled); }

        // 画像をデコードからピクセルデータへ変換
        let mut image = DynamicImage::from_decoder(decoder).map_err(|e| {
            error::SungaError::resize_error(e.to_string(), path.to_path_buf())
        })?;

        // 画像の向きを適用
        image.apply_orientation(orientation);

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(resize::ExecuteResult::Canceled); }

        // リサイズをスキップするかどうかを判定
        if Self::should_skip(&image, options) {
            // 出力ファイルが存在しない場合は作成、エラーが発生した場合はエラーを返す
            match output_path.try_exists() {
                Ok(true) => (),
                Ok(false) => {
                    Self::make_output_path(output_path)?;

                    // ファイルをコピー
                    std::fs::copy(path, output_path).map_err(|e| {
                        error::SungaError::resize_error(e.to_string(), output_path.to_path_buf())
                    })?;
                }
                Err(e) => {
                    return Err(error::SungaError::resize_error(e.to_string(), output_path.to_path_buf()));
                }
            }

            return Ok(resize::ExecuteResult::Unchanged);
        }

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(resize::ExecuteResult::Canceled); }

        // リサイズを実行
        let resized = Self::resize(&image, options);

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(resize::ExecuteResult::Canceled); }

        // 出力パスを作成
        Self::make_output_path(output_path)?;
        // 一時ファイルのパスを作成
        let temp_path = output_path.with_added_extension(TEMP_EXTENSION);

        // キャンセルされた場合は処理を中断
        if token.is_canceled()? { return Ok(resize::ExecuteResult::Canceled); }

        // 一時ファイルを削除するためのガードを作成
        let mut guard = ResizeGuard::new(temp_path.clone());

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

        Ok(resize::ExecuteResult::Success { width: resized.width(), height: resized.height() })
    }
}

/// private methods
impl Resize {
    /// リサイズ
    /// * `image` - リサイズする画像
    /// * `options` - リサイズオプション
    /// * `return` - リサイズした画像
    fn resize(image: &DynamicImage, options: &resize::Options) -> DynamicImage {
        // リサイズ後のサイズを計算
        let (width, height) = Self::calculate_resized_size(image, options);

        // 画像のフィルター種類を取得
        let filter = match options.filter {
            resize::Filter::Lanczos3 => image::imageops::FilterType::Lanczos3,
            resize::Filter::CatmullRom => image::imageops::FilterType::CatmullRom,
            resize::Filter::Triangle => image::imageops::FilterType::Triangle,
            resize::Filter::Gaussian => image::imageops::FilterType::Gaussian,
            resize::Filter::Nearest => image::imageops::FilterType::Nearest,
        };

        // 画像をリサイズ
        image.resize_exact(width, height, filter)
    }

    /// リサイズをスキップするかどうかを判定
    ///
    /// 両方時：元の画像とリサイズ後のサイズが同じ
    /// 拡大時：元の画像がリサイズ後のサイズ以上
    /// 縮小時：元の画像がリサイズ後のサイズ以下
    /// の場合は true を返す
    ///
    /// * `image` - 画像
    /// * `options` - リサイズオプション
    /// * `return` - リサイズをスキップするかどうか
    fn should_skip(image: &DynamicImage, options: &resize::Options) -> bool {
        // リサイズ後のサイズを計算
        let (width, height) = Self::calculate_resized_size(image, options);

        // 指定されているリサイズの仕方によって、それぞれの判定を行う
        match options.resize_policy {
            resize::ResizePolicy::Both => image.width() == width && image.height() == height,
            resize::ResizePolicy::Enlarge => image.width() >= width && image.height() >= height,
            resize::ResizePolicy::Shrink => image.width() <= width && image.height() <= height,
        }
    }

    /// リサイズ後のサイズを計算
    /// * `image` - 元の画像
    /// * `options` - リサイズオプション
    /// * `return` - リサイズ後のサイズ
    fn calculate_resized_size(image: &DynamicImage, options: &resize::Options) -> (u32, u32) {
        let (orig_width, orig_height) = image.dimensions();

        match options.target {
            resize::Target::Size { width, height } => {
                Self::calculate_size((orig_width, orig_height), (width, height))
            }
            resize::Target::Scale(scale) => {
                let (width, height) = Self::calculate_scale((orig_width, orig_height), scale);
                Self::calculate_size((orig_width, orig_height), (width, height))
            }
        }
    }

    /// 画像のサイズを計算
    /// * `image_size` - 画像のサイズ
    /// * `target_size` - リサイズ後のサイズ
    /// * `return` - リサイズ後のサイズ
    fn calculate_size(image_size: (u32, u32), target_size: (u32, u32)) -> (u32, u32) {
        let (image_width, image_height) = image_size;
        let (target_width, target_height) = target_size;

        // 比率を計算
        let ratio = (target_width as f64 / image_width as f64).min(target_height as f64 / image_height as f64);

        // 幅と高さを計算
        let width = (image_width as f64 * ratio).round().max(1.0) as u32;
        let height = (image_height as f64 * ratio).round().max(1.0) as u32;

        (width, height)
    }

    /// スケールを計算
    /// * `image_size` - 画像のサイズ
    /// * `scale` - スケール
    /// * `return` - スケールしたサイズ
    fn calculate_scale(image_size: (u32, u32), scale: u16) -> (u32, u32) {
        let (image_width, image_height) = image_size;

        // 幅と高さを計算
        let width = (image_width as f64 * scale as f64 / 100.0).round().max(1.0) as u32;
        let height = (image_height as f64 * scale as f64 / 100.0).round().max(1.0) as u32;

        (width, height)
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
        // EXIF の Orientation をリセット
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
