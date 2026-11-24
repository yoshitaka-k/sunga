mod handler;
pub(crate) mod button;
pub(crate) mod click;
pub(crate) mod input;
pub(crate) mod key;
pub(crate) mod open;

pub(crate) use handler::handle;

/// イベントアクション
pub(crate) enum EventAction {
    #[allow(dead_code)]
    FilesOpen,
    FolderOpen,
    DropOpen(Vec<std::path::PathBuf>),
    Clear,
    SettingOpen(Option<egui::Pos2>),
    SettingClose,
    Click { index: Option<usize> },
    DoubleClick(std::path::PathBuf),
    Up,
    Down,
    Enter(std::path::PathBuf),
    Space(std::path::PathBuf),
    Backspace,
    CheckForUpdate,
}
