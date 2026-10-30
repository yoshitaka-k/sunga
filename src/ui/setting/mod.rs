mod view;
mod top;
mod general;
mod resize;
mod concurrent;
mod output;
mod about;

pub(crate) use view::view;

use crate::{event, ui, lang};
use crate::ui::assets::{self, icon, svg};

// ウィンドウのID
const SETTING_WINDOW_ID: &str = "setting_window";

// ウィンドウのサイズ
const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 260.0;

// トップパネルの名前
const TOP_PANEL_NAME: &str = "tab_panel";

// パネルのスペースの幅
const ADD_SPACING: f32 = 4.0;

// ヘッダーのスペースの幅
const HEADER_ICON_SPACING: f32 = 4.0;
const WARNING_ICON_SPACING: f32 = 3.0;
const SETTING_INDENT_SPACING: f32 = 10.0;
const RESIZE_INDENT_SPACING: f32 = 8.0;

// ラベルの幅
const GENERAL_PATH_LABEL_WIDTH: f32 = 30.0;
const GENERAL_LABEL_WIDTH: f32 = 94.0;
const RESIZE_LABEL_WIDTH: f32 = 60.0;
const CONCURRENT_LABEL_WIDTH: f32 = 80.0;

// 並行処理数のスライダーの最小値と最大値
pub(crate) const CONCURRENT_NUM_MIN: u8 = 1;
pub(crate) const CONCURRENT_NUM_MAX: u8 = 8;

// 効果音の音量のスライダーの最小値と最大値
pub(crate) const SOUND_VOLUME_MIN: u8 = 1;
pub(crate) const SOUND_VOLUME_MAX: u8 = 10;

/// 設定ウィンドウのヘッダーパネルを表示
/// * `ui` - UI
/// * `icon` - アイコン
/// * `label` - ラベル
/// * `pending_actions` - 処理予約
/// * `is_update_check` - アップデート確認ボタンを表示するかどうか
pub(crate) fn header_panel(
    ui: &mut egui::Ui,
    icon: egui::ImageSource<'static>,
    label: &str,
    pending_actions: &mut Vec<event::EventAction>,
    is_update_check: bool,
) {
    let spacing = ui.spacing().item_spacing.x;

    // ボタンの色を設定
    let icon_color = ui::icon_color(ui);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = HEADER_ICON_SPACING;
        ui.add(egui::Image::new(icon).max_height(icon::SETTINGS_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(label);

        // アップデート確認ボタン
        if is_update_check {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(lang::current().setting_about.update_check).clicked() {
                    pending_actions.push(event::EventAction::CheckForUpdate);
                }
            });
        }
    });
}

/// グループヘッダーパネルを表示
/// * `ui` - UI
pub(crate) fn group_header_panel(
    ui: &mut egui::Ui,
    label: &str,
) {
    ui.horizontal(|ui| {
        ui.add_space(SETTING_INDENT_SPACING);
        ui.label(label);
    });
}

/// 警告ノートを表示
/// * `ui` - UI
/// * `text` - 警告ノートのテキスト
/// * `return` - 警告ノートのリッチテキスト
pub(crate) fn warning_note(
    ui: &mut egui::Ui,
    text: &str
) {
    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // 出力パスの注意書きを表示
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = WARNING_ICON_SPACING;
        ui::add_icon(ui, svg::WARNING, icon::WARNING_ICON_SIZE, assets::warning_color(ui));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.add(egui::Label::new(
            egui::RichText::new(text).weak(),
        ));
    });
}

/// ラベル後の残り幅に合わせてスライダーのレール幅を決める
/// * `ui` - UI
/// * `return` - スライダーのレール幅
pub(crate) fn remaining_slider_width(ui: &egui::Ui) -> f32 {
    let spacing = ui.spacing();
    (ui.available_width() - spacing.item_spacing.x - spacing.interact_size.x).max(0.0)
}
