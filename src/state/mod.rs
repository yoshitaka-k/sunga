mod token;

pub(crate) use token::ErrorToken;
pub(crate) use token::ScrollToken;
pub(crate) use token::SettingToken;

use crate::{config, file, resize, update};

/// アプリケーションの状態
pub(crate) struct State {
    pub config: config::Config,
    pub files: file::OpenFiles,
    pub setting_token: SettingToken,
    pub updated_token: update::UpdatedToken,
    pub error_token: ErrorToken,
    pub scroll_token: ScrollToken,
}

/// ジョブ
pub(crate) struct Jobs {
    pub resize_job: resize::ResizeJob,
    pub update_job: update::UpdateJob,
}

/// 設定ウィンドウのタブ
/// * `General` - 一般
/// * `Resize` - リサイズ
/// * `Output` - 出力
/// * `Concurrent` - 同時処理
/// * `About` - アバウト
#[derive(PartialEq)]
pub(crate) enum SettingTab {
    General,
    Resize,
    Output,
    Concurrent,
    About,
}
