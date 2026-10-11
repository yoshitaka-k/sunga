pub(crate) mod button;
pub(crate) mod input;
pub(crate) mod drop;
pub(crate) mod open;

/// イベントアクション
pub(crate) enum EventAction {
    FilesOpen,
    FolderOpen,
    SettingOpen,
    Clear,
}
