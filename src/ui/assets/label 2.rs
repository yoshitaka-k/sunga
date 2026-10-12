/// メイントップのラベル
pub(crate) struct MainTop;
impl MainTop {
    /// 設定ボタンのラベル
    /// * `return` - 設定ボタンのラベル
    pub fn settings() -> &'static str {
        "Settings"
    }

    /// ファイルダイアログを開くボタンのラベル
    /// * `return` - ファイルダイアログを開くボタンのラベル
    pub fn file_open() -> &'static str {
        "File Open"
    }

    /// フォルダダイアログを開くボタンのラベル
    /// * `return` - フォルダダイアログを開くボタンのラベル
    pub fn folder_open() -> &'static str {
        "Folder Open"
    }
}

/// メインボトムのラベル
pub(crate) struct MainBottom;
impl MainBottom {
    /// クリアボタンのラベル
    /// * `return` - クリアボタンのラベル
    pub fn clear() -> &'static str {
        "Clear"
    }
}
