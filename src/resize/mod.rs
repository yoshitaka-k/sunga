mod job;
mod resize;
mod token;
mod guard;
pub(crate) use job::ResizeJob;
pub(crate) use resize::Resize;
pub(crate) use token::Token;
pub(crate) use guard::ResizeGuard;

use serde::{Deserialize, Serialize};
use std::fmt;
use crate::lang;

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

/// リサイズの種類
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) enum Mode {
    Size,
    Scale,
}
impl Mode {
    pub fn is_size(&self) -> bool {
        matches!(self, Mode::Size)
    }
    pub fn is_scale(&self) -> bool {
        matches!(self, Mode::Scale)
    }
}

/// 画像のフィルター種類
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) enum Filter {
    Lanczos3,
    CatmullRom,
    Triangle,
    Gaussian,
    Nearest,
}
impl fmt::Display for Filter {
    /// 画像のフィルター種類を文字列に変換
    /// * `return` - 画像のフィルター種類を文字列に変換
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let filter = &lang::current().resize_filter;
        match self {
            Filter::Lanczos3 => write!(f, "{}", filter.lanczos3),
            Filter::CatmullRom => write!(f, "{}", filter.catmull_rom),
            Filter::Triangle => write!(f, "{}", filter.triangle),
            Filter::Gaussian => write!(f, "{}", filter.gaussian),
            Filter::Nearest => write!(f, "{}", filter.nearest),
        }
    }
}

/// 画像のフィルター種類のイテレータ
/// * `return` - 画像のフィルター種類のイテレータ
pub(crate) fn filter_iter() -> impl Iterator<Item = Filter> {
    vec![
        Filter::Lanczos3,
        Filter::CatmullRom,
        Filter::Triangle,
        Filter::Gaussian,
        Filter::Nearest,
    ].into_iter()
}

/// リサイズの対象
/// * `Size` - 幅と高さ
/// * `Scale` - 倍率
pub(crate) enum Target {
    Size { width: u32, height: u32 },
    Scale(u16),
}
impl Target {
    pub fn validate(&self) -> Option<String> {
        match self {
            Target::Size { width, height } => {
                if !(SIZED_RANGE_MIN..=SIZED_RANGE_MAX).contains(&width) {
                    Some(format!("Width is out of range ({}..={})", SIZED_RANGE_MIN, SIZED_RANGE_MAX))
                } else if !(SIZED_RANGE_MIN..=SIZED_RANGE_MAX).contains(&height) {
                    Some(format!("Height is out of range ({}..={})", SIZED_RANGE_MIN, SIZED_RANGE_MAX))
                } else {
                    None
                }
            }
            Target::Scale(scale) => {
                if !(SCALE_RANGE_MIN..=SCALE_RANGE_MAX).contains(&scale) {
                    Some(format!("Scale is out of range ({}..={})", SCALE_RANGE_MIN, SCALE_RANGE_MAX))
                } else {
                    None
                }
            }
        }
    }
}

/// リサイズオプション
/// * `target` - リサイズの対象
pub(crate) struct Options {
    pub target: Target,
    pub filter: Filter,
}
