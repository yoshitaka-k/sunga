mod job;
mod executor;
mod token;
mod guard;
pub(crate) use job::ResizeJob;
pub(crate) use executor::Resize;
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

/// リサイズの種類（拡大・縮小・両方）
/// * `Enlarge` - 拡大
/// * `Shrink` - 縮小
/// * `Both` - 両方
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) enum Direction {
    Enlarge,
    Shrink,
    Both,
}

/// リサイズする時の対象の種類
/// * `Size` - サイズ指定
/// * `Scale` - 倍率指定
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) enum TargetKind {
    Size,
    Scale,
}
impl TargetKind {
    pub fn is_size(&self) -> bool {
        matches!(self, TargetKind::Size)
    }
    pub fn is_scale(&self) -> bool {
        matches!(self, TargetKind::Scale)
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
#[derive(Clone)]
pub(crate) enum Target {
    Size { width: u32, height: u32 },
    Scale(u16),
}
impl Target {
    pub fn validate(&self) -> Option<String> {
        match self {
            Target::Size { width, height } => {
                if !(SIZED_RANGE_MIN..=SIZED_RANGE_MAX).contains(width) {
                    Some(format!("Width is out of range ({}..={})", SIZED_RANGE_MIN, SIZED_RANGE_MAX))
                } else if !(SIZED_RANGE_MIN..=SIZED_RANGE_MAX).contains(height) {
                    Some(format!("Height is out of range ({}..={})", SIZED_RANGE_MIN, SIZED_RANGE_MAX))
                } else {
                    None
                }
            }
            Target::Scale(scale) => {
                if !(SCALE_RANGE_MIN..=SCALE_RANGE_MAX).contains(scale) {
                    Some(format!("Scale is out of range ({}..={})", SCALE_RANGE_MIN, SCALE_RANGE_MAX))
                } else {
                    None
                }
            }
        }
    }
}

/// リサイズオプション
/// * `direction` - リサイズの方向（拡大・縮小・両方）
/// * `target` - リサイズの対象（サイズ指定・倍率指定）
/// * `filter` - 画像のフィルター種類
#[derive(Clone)]
pub(crate) struct Options {
    #[expect(dead_code, reason = "拡大・縮小の切り替えは実装予定")]
    pub direction: Direction,
    pub target: Target,
    pub filter: Filter,
}
