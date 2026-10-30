pub(crate) mod handler;
pub(crate) mod button;
pub(crate) mod input;
pub(crate) mod open;
pub(crate) mod click;
pub(crate) mod key;

/// イベントアクション
pub(crate) enum EventAction {
    #[allow(dead_code)]
    FilesOpen,
    FolderOpen,
    DropOpen(Vec<std::path::PathBuf>),
    Clear,
    Click { index: Option<usize> },
    DoubleClick(std::path::PathBuf),
    Enter(std::path::PathBuf),
    Space(std::path::PathBuf),
    Backspace,
    Up,
    Down,
    SettingOpen(Option<egui::Pos2>),
    SettingClose,
    CheckForUpdate,
}
