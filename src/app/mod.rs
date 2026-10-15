mod update;
pub(crate) use update::{UpdateJob, UpdatedToken};

use getset::Getters;
use serde::{Deserialize, Serialize};

const DEFAULT_WIDTH: u32 = 2560;
const DEFAULT_HEIGHT: u32 = 1440;
const DEFAULT_SCALE: f32 = 1.0;
const DEFAULT_RESIZE_NUM: u8 = 4;

/// 状態管理アプリケーション
#[derive(Clone, Getters, Deserialize, Serialize)]
#[serde(default)]
#[getset(get = "pub")]
pub struct App {
    /// リサイズする幅・高さ
    width: u32,
    height: u32,

    /// リサイズする倍率
    scale: f32,

    /// リサイズできるファイル数
    resize_num: u8,

    /// 同じパスはスキップ
    skip_same_path: bool,

    /// 出力パス
    output_path: String,
}

impl Default for App {
    /// デフォルト値を設定
    /// * `return` - App のインスタンス
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            scale: DEFAULT_SCALE,
            resize_num: DEFAULT_RESIZE_NUM,
            skip_same_path: true,
            output_path: "".to_string(),
        }
    }
}

impl App {
    /// 新しい App を作成
    /// * `return` - App のインスタンス
    pub fn new() -> Self {
        Self::default()
    }
}
