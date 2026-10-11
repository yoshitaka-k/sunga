mod assets;
mod main;
mod setting;

pub use main::render::Render;

use crate::ui::assets::color;

// パネルの内側のマージン
pub(crate) const TOP_PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 10, right: 10, top: 4, bottom: 4,
};
pub(crate) const BOTTOM_PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 10, right: 10, top: 4, bottom: 6,
};

/// ファイルダイアログを表示するためのトークン
pub(crate) struct DialogToken {
    pub file: bool,
    pub folder: bool,
}
impl DialogToken {
    pub fn new() -> Self {
        Self {
            file: false,
            folder: false,
        }
    }
}

/// 設定ウィンドウのタブ
#[derive(PartialEq)]
pub enum SettingTab {
    General,
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

/// パネルのスタイルを設定
/// * `ui` - UI
/// * `inner_margin` - パネルの内側のマージン
/// * `return` - パネルのスタイル
pub(crate) fn panel_style(ui: &mut egui::Ui, inner_margin: egui::Margin) -> egui::Frame {
    let panel_fill_color = panel_fill_color(ui);
    egui::Frame::default()
        .fill(panel_fill_color)
        .inner_margin(inner_margin)
}

/// パネルの背景色
/// * `ui` - UI
/// * `return` - パネルの背景色
fn panel_fill_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        color::DARK_MODE_PANEL_COLOR
    } else {
        color::LIGHT_MODE_PANEL_COLOR
    }
}
