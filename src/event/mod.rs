pub(crate) mod button;
pub(crate) mod input;
pub(crate) mod drop;
pub(crate) mod open;
pub(crate) mod click;

/// イベントアクション
pub(crate) enum EventAction {
    #[allow(dead_code)]
    FilesOpen,

    FolderOpen,
    SettingOpen,
    Clear,
    Click { id: u64 },
    DoubleClick(std::path::PathBuf),
}
