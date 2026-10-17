mod update;
pub(crate) use update::{UpdateJob, UpdatedToken};

use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};

const DEFAULT_WIDTH: u32 = 2560;
const DEFAULT_HEIGHT: u32 = 1440;
const DEFAULT_SCALE: f32 = 2.0;
const DEFAULT_RESIZE_NUM: u8 = 4;

/// 状態管理アプリケーション
#[derive(Clone, Getters, MutGetters, Deserialize, Serialize)]
#[serde(default)]
#[getset(get = "pub", get_mut = "pub")]
pub struct App {
    /// リサイズする幅・高さ
    width: u32,
    height: u32,

    /// リサイズする倍率
    scale: f32,

    /// 倍率を使用するかどうか
    is_scale: bool,

    /// リサイズできるファイル数
    resize_num: u8,

    /// 出力パス
    output_path: String,

    /// 同じパスはスキップ
    skip_same_path: bool,
}

impl Default for App {
    /// デフォルト値を設定
    /// * `return` - App のインスタンス
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            scale: DEFAULT_SCALE,
            is_scale: false,
            resize_num: DEFAULT_RESIZE_NUM,
            output_path: "".to_string(),
            skip_same_path: true,
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
