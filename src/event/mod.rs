pub(crate) mod button;
pub(crate) mod drop;
pub(crate) mod input;

/// イベントアクション
pub(crate) enum EventAction {
    SettingOpen,
    FolderOpen,
    FilesOpen,
    Clear,
}
