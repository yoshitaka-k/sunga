mod assets;
mod main;
mod setting;
mod modal;

/// メイン描画関数を公開
pub(crate) use main::view as draw;

use serde::{Deserialize, Serialize};
use crate::file;
use crate::ui::assets::{color, font, svg};

// メインビューの const

/// トップメニューバーの名前
const TOP_PANEL_NAME: &str = "top_menubar";
/// 下部メニューバーの名前
const BOTTOM_PANEL_NAME: &str = "bottom_menubar";
/// セパレータの高さ
const SEPARATOR_HEIGHT: f32 = 4.0;
/// リストアイコンのインデント
const LIST_ICON_INDENT: f32 = 2.0;
/// リストの角の半径
const LIST_CORNER_RADIUS: f32 = 4.0;
/// リストの名前の最小比率
const LIST_NAME_MIN_RATIO: f32 = 1.0 / 3.3;
/// リストの右インデント
const LIST_RIGHT_INDENT: f32 = 10.0;

// 設定ウィンドウの const

/// 設定ウィンドウのID
const SETTING_WINDOW_ID: &str = "setting_window";
/// タブパネルの名前
const TAB_PANEL_NAME: &str = "tab_panel";
/// 設定ウィンドウの幅
const SETTING_WIDTH: f32 = 480.0;
/// 設定ウィンドウの高さ
const SETTING_HEIGHT: f32 = 290.0;
/// 追加のスペース
const ADD_SPACING: f32 = 4.0;
/// ヘッダーアイコンのスペース
const HEADER_ICON_SPACING: f32 = 4.0;
/// 警告アイコンのスペース
const WARNING_ICON_SPACING: f32 = 3.0;
/// 設定インデントのスペース
const SETTING_INDENT_SPACING: f32 = 10.0;
/// リサイズインデントのスペース
const RESIZE_INDENT_SPACING: f32 = 8.0;
/// 一般パスラベルの幅
const GENERAL_PATH_LABEL_WIDTH: f32 = 30.0;
/// 一般ラベルの幅
const GENERAL_LABEL_WIDTH: f32 = 94.0;
/// リサイズラベルの幅
const RESIZE_LABEL_WIDTH: f32 = 40.0;
/// リサイズラベルの最小幅
const RESIZE_LABEL_MIN_WIDTH: f32 = 30.0;
/// 並行処理数ラベルの幅
const CONCURRENT_LABEL_WIDTH: f32 = 80.0;

// モーダルの const

/// モーダルウィンドウの幅
const MODAL_WIDTH: f32 = 300.0;
/// モーダルウィンドウのスペース
const MODAL_SPACING: f32 = 8.0;
/// モーダルラベルの幅
const MODAL_LABEL_WIDTH: f32 = 130.0;

// パネルの内側のマージン

/// トップメニューバーの内側のマージン
const TOP_PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 10, right: 10, top: 4, bottom: 4,
};
/// 下部メニューバーの内側のマージン
const BOTTOM_PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 10, right: 10, top: 4, bottom: 6,
};
/// パネルの内側のマージン
const PANEL_INNER_MARGIN: egui::Margin = egui::Margin {
    left: 8, right: 8, top: 0, bottom: 0,
};

/// テーマ
#[derive(Clone, Copy, PartialEq, Deserialize, Serialize)]
pub(crate) enum Theme {
    Dark,
    Light,
}

/// ステータスアイコンの色
pub(crate) struct StatusColor {
    standby: egui::Color32,
    resizing: egui::Color32,
    resized: egui::Color32,
    skipped: egui::Color32,
    unchanged: egui::Color32,
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
            unchanged: assets::status_color(ctx, file::Status::Unchanged),
            canceled: assets::status_color(ctx, file::Status::Canceled),
            error: assets::status_color(ctx, file::Status::Error(String::new())),
        }
    }
}

pub(crate) fn install(ctx: &egui::Context) {
    // フォントと SVG ローダーを追加
    font::install(ctx);
    svg::install(ctx);
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

/// テキストカラーを設定する
/// * `ui` - UI
/// * `text` - テキスト
/// * `color` - カラー
/// * `return` - ラベルのレスポンス
pub(crate) fn rich_label(ui: &mut egui::Ui, text: &str, color: egui::Color32) -> egui::Response {
    ui.label(egui::RichText::new(text).color(color))
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
