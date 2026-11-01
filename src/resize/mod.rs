mod job;
mod resize;
mod token;
mod option;
mod guard;
pub(crate) use job::ResizeJob;
pub(crate) use resize::Resize;
pub(crate) use token::Token;
pub(crate) use option::Options;
pub(crate) use guard::ResizeGuard;

/// 一時ファイルの拡張子
pub(crate) const TEMP_EXTENSION: &str = "sunga.temp";

/// JPEG の品質
pub(crate) const JPEG_QUALITY: u8 = 95;

/// サイズの範囲
pub(crate) const SIZED_RANGE_MIN: u32 = 10;
pub(crate) const SIZED_RANGE_MAX: u32 = 5120;

/// 倍率の範囲
pub(crate) const SCALE_RANGE_MIN: u16 = 10;
pub(crate) const SCALE_RANGE_MAX: u16 = 200;
