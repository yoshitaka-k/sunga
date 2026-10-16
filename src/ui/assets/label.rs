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
        "Cancel and Clear"
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

pub(crate) struct SettingGeneral;
impl SettingGeneral {
    /// 出力先のラベル
    /// * `return` - 出力先のラベル
    pub fn header_output() -> &'static str {
        "Output"
    }

    /// パスのラベル
    /// * `return` - パスのラベル
    pub fn path() -> &'static str {
        "Path:"
    }

    /// ブラウズボタンのラベル
    /// * `return` - ブラウズボタンのラベル
    pub fn browse() -> &'static str {
        "Browse"
    }

    /// クリアボタンのラベル
    /// * `return` - クリアボタンのラベル
    pub fn clear() -> &'static str {
        "Clear"
    }

    /// 出力先の注意書きのラベル
    /// * `return` - 出力先の注意書きのラベル
    pub fn output_warning_note() -> &'static str {
        "Leave empty to overwrite the original files."
    }

    /// 同じパスはスキップのラベル
    /// * `return` - 同じパスはスキップのラベル
    pub fn skip_same_path() -> &'static str {
        "Skip same path:"
    }

    /// 同じパスはスキップのオプションのラベル
    /// * `return` - 同じパスはスキップのオプションのラベル
    pub fn skip_same_path_option() -> &'static str {
        "Skip same path"
    }

    /// 同じパスはスキップのオプションのラベル
    /// * `return` - 同じパスはスキップのオプションのラベル
    pub fn dont_skip_same_path_option() -> &'static str {
        "Don't skip same path"
    }

    /// 同じパスはスキップの注意書きのラベル
    /// * `return` - 同じパスはスキップの注意書きのラベル
    pub fn skip_same_path_warning_note() -> &'static str {
        "Skip if already completed. Retry canceled and errors."
    }
}

/// アバウトウィンドウのラベル
pub(crate) struct SettingAbout;
impl SettingAbout {
    /// アバウトウィンドウのタイトル
    /// * `return` - アバウトウィンドウのタイトル
    pub fn heading() -> &'static str {
        "About"
    }

    /// アップデート確認ボタンのラベル
    /// * `return` - アップデート確認ボタンのラベル
    pub fn update_check() -> &'static str {
        "Check for Update"
    }
}

/// ファイルダイアログのラベル
pub(crate) struct FileDialog;
impl FileDialog {
    /// ファイルダイアログのタイトル
    /// * `return` - ファイルダイアログのタイトル
    pub fn image_files() -> &'static str {
        "Image Files"
    }
}
