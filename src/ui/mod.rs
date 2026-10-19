mod assets;
mod main;
mod setting;

pub use main::render::Render;

use crate::file;
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
    Resize,
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

/// ステータスアイコンの色
pub(crate) struct StatusColor {
    standby: egui::Color32,
    resizing: egui::Color32,
    resized: egui::Color32,
    skipped: egui::Color32,
    canceled: egui::Color32,
    error: egui::Color32,
}
impl StatusColor {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            standby: assets::status_color(ctx, file::Status::Standby),
            resizing: assets::status_color(ctx, file::Status::Resizing),
            resized: assets::status_color(ctx, file::Status::Resized),
            skipped: assets::status_color(ctx, file::Status::Skipped),
            canceled: assets::status_color(ctx, file::Status::Canceled),
            error: assets::status_color(ctx, file::Status::Error(String::new())),
        }
    }
}

/// アイコンを追加
/// * `ui` - UI
/// * `icon` - アイコンのパス
/// * `size` - アイコンのサイズ
/// * `color` - アイコンの色
/// * `return` - アイコンのUI
pub(crate) fn add_icon(
    ui: &mut egui::Ui,
    icon: egui::ImageSource<'static>,
    size: f32,
    color: egui::Color32,
) -> egui::Response {
    ui.add(
        egui::Image::new(icon)
            .max_height(size)
            .tint(color)
    )
}

/// ラベルの幅を取得
/// * `ui` - UI
/// * `label` - ラベルのテキスト
/// * `width` - ラベルの幅
/// * `return` - ラベルのレスポンス
pub(crate) fn add_label(
    ui: &mut egui::Ui,
    label: &str,
    width: f32
) -> egui::Response {
    ui.allocate_ui_with_layout(
        egui::vec2(width, ui.spacing().interact_size.y),
        egui::Layout::right_to_left(egui::Align::Center),
        |ui| {
            ui.set_min_width(width);
            ui.label(label)
        }
    ).inner
}

/// ラベルと同じ色を返す
/// * `ui` - UI
/// * `return` - アイコンの色
pub(crate) fn icon_color(ui: &egui::Ui) -> egui::Color32 {
    ui.visuals().text_color()
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
