pub(crate) mod button;
pub(crate) mod input;
pub(crate) mod drop;
pub(crate) mod open;
pub(crate) mod click;
pub(crate) mod key;

/// イベントアクション
pub(crate) enum EventAction {
    #[allow(dead_code)]
    FilesOpen,

    FolderOpen,
    SettingOpen,
    Clear,
    Click { id: u64 },
    DoubleClick(std::path::PathBuf),
    Enter(std::path::PathBuf),
    Space(std::path::PathBuf),
    Backspace,
    Up { id: u64 },
    Down { id: u64 },
}
