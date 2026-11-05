use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};
use crate::{resize, ui};

const DEFAULT_WIDTH: u32 = 2560;
const DEFAULT_HEIGHT: u32 = 1440;
const DEFAULT_SCALE: u16 = 100;
const DEFAULT_CONCURRENT_NUM: u8 = 4;
const DEFAULT_SOUND_VOLUME: u8 = 5;

/// アプリケーションの設定
#[derive(Clone, Getters, MutGetters, Deserialize, Serialize)]
#[serde(default)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Config {
    /// リサイズする幅・高さ
    width: u32,
    height: u32,

    /// リサイズする倍率
    scale: u16,

    /// リサイズの種類
    target_kind: resize::TargetKind,

    /// 画像のフィルター種類
    filter: resize::Filter,

    /// 並行処理できる最大数
    concurrent_num: u8,

    /// 出力パス
    output_path: String,

    /// 同じパスはスキップ
    skip_same_path: bool,

    /// サウンドを再生するかどうか
    play_sound: bool,

    /// サウンドの音量
    sound_volume: u8,

    /// 言語
    language: String,

    /// テーマ
    theme: ui::Theme,
}

impl Default for Config {
    /// デフォルト値を設定
    /// * `return` - Config のインスタンス
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            scale: DEFAULT_SCALE,
            target_kind: resize::TargetKind::Size,
            filter: resize::Filter::Lanczos3,
            concurrent_num: DEFAULT_CONCURRENT_NUM,
            output_path: "".to_string(),
            skip_same_path: true,
            play_sound: true,
            sound_volume: DEFAULT_SOUND_VOLUME,
            language: "en".to_string(),
            theme: ui::Theme::Dark,
        }
    }
}

impl Config {
    /// 新しい Config を作成
    /// * `return` - Config のインスタンス
    pub fn new() -> Self {
        Self::default()
    }

    /// リサイズの対象を取得
    /// * `return` - リサイズの対象
    pub(crate) fn resize_target(&self) -> resize::Target {
        match *self.target_kind() {
            resize::TargetKind::Size => resize::Target::Size { width: *self.width(), height: *self.height() },
            resize::TargetKind::Scale => resize::Target::Scale(*self.scale()),
        }
    }
}
