use crate::error;

/// エラーモーダルを表示するためのトークン
pub(crate) struct ErrorToken {
    pub open: bool,
    pub value: Option<error::SungaError>,
    pub dismissed: bool,
}
impl ErrorToken {
    pub fn new() -> Self {
        Self {
            open: false,
            value: None,
            dismissed: false,
        }
    }

    /// エラーモーダルを表示
    /// * `error` - エラー内容
    pub fn show(&mut self, error: error::SungaError) {
        if self.dismissed {
            return;
        }

        // 同じ失敗が毎フレーム来ても、ログは最初の1回だけ出す
        if self.value.is_none() {
            let (file, line) = error.location();
            log::error!("{error} @ {file}:{line}");
        }

        self.open = true;
        self.value = Some(error);
    }

    /// エラーモーダルを非表示
    pub fn dismiss(&mut self) {
        self.open = false;
        self.value = None;
        self.dismissed = true;
    }

    /// エラーモーダルをリセット
    pub fn reset(&mut self) {
        self.open = false;
        self.value = None;
        self.dismissed = false;
    }
}

/// スクロールエリアを表示するためのトークン
pub(crate) struct ScrollToken {
    pub selected: bool,
}
impl ScrollToken {
    pub fn new() -> Self {
        Self {
            selected: false,
        }
    }
}

/// 設定ウィンドウのタブ
#[derive(PartialEq)]
pub(crate) enum SettingTab {
    General,
    Concurrent,
    Resize,
    About,
}

/// 設定ウィンドウを表示するためのデータ
pub(crate) struct SettingToken {
    pub open: bool,
    pub pos: Option<egui::Pos2>,
    pub tab: SettingTab,
}
impl SettingToken {
    pub fn new() -> Self {
        Self {
            open: false,
            pos: None,
            tab: SettingTab::General,
        }
    }
}
