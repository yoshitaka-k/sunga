mod update;
pub(crate) use update::{UpdateJob, UpdatedToken};

const DEFAULT_WIDTH: u32 = 0;
const DEFAULT_HEIGHT: u32 = 0;

/// 状態管理アプリケーション
pub struct App {
    width: u32,
    height: u32,
}

impl App {
    /// 新しい App を作成
    /// * `return` - App のインスタンス
    pub fn new() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
    }
}
