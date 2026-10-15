mod view;
mod top;
mod general;
mod about;

pub(crate) use view::view;

use crate::{app, ui};
use crate::event::button;
use crate::ui::assets::{self, label, icon, svg};

// ウィンドウのID
const SETTING_WINDOW_ID: &str = "setting_window";

// ウィンドウのサイズ
const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 340.0;

// トップパネルの名前
const TOP_PANEL_NAME: &str = "tab_panel";

// ヘッダーのスペースの幅
const HEADER_ICON_SPACING: f32 = 4.0;
const WARNING_ICON_SPACING: f32 = 3.0;
const SETTING_INDENT_SPACING: f32 = 10.0;

// ラベルの幅
pub(crate) const GENERAL_LABEL_WIDTH: f32 = 50.0;

/// 設定ウィンドウのヘッダーパネルを表示
/// * `ui` - UI
/// * `icon` - アイコン
/// * `label` - ラベル
/// * `update_job` - アップデートジョブ
pub(crate) fn header_panel(
    ui: &mut egui::Ui,
    icon: egui::ImageSource<'static>,
    label: &str,
    update_job: Option<&mut app::UpdateJob>,
) {
    let spacing = ui.spacing().item_spacing.x;

    // ボタンの色を設定
    let icon_color = assets::icon_color(ui);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = HEADER_ICON_SPACING;
        ui.add(egui::Image::new(icon).max_height(icon::SETTINGS_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(label);

        // アップデート確認ボタン
        if let Some(update_job) = update_job {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(label::SettingAbout::update_check()).clicked() {
                    button::check_for_update(update_job);
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
