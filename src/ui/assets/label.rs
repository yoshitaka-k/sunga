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
    pub fn files_open() -> &'static str {
        "Files Open"
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

/// 設定ウィンドウのラベル
pub(crate) struct Setting;
impl Setting {
    /// 設定ウィンドウのタイトル
    /// * `return` - 設定ウィンドウのタイトル
    pub fn heading() -> &'static str {
        "Settings"
    }

    /// 一般タブのラベル
    /// * `return` - 一般タブのラベル
    pub fn general() -> &'static str {
        "General"
    }

    /// アバウトタブのラベル
    /// * `return` - アバウトタブのラベル
    pub fn about() -> &'static str {
        "About"
    }
}
