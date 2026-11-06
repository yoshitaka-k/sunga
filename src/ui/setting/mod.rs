mod view;
mod top;
mod general;
mod resize;
mod concurrent;
mod output;
mod about;

pub(crate) use view::view;

use crate::{lang, event, ui};
use crate::ui::assets::{self, icon, svg};

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
        ui.spacing_mut().item_spacing.x = ui::HEADER_ICON_SPACING;
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
        ui.add_space(ui::SETTING_INDENT_SPACING);
        ui.label(label);
    });
}

/// 警告ノートを表示
/// * `ui` - UI
/// * `text` - 警告ノートのテキスト
/// * `space` - スペースの幅
pub(crate) fn warning_note(
    ui: &mut egui::Ui,
    text: &str,
    space: Option<f32>,
) {
    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // 出力パスの注意書きを表示
    ui.horizontal(|ui| {
        if let Some(space) = space {
            ui.add_space(space);
        }

        ui.spacing_mut().item_spacing.x = ui::WARNING_ICON_SPACING;
        ui::add_icon(ui, svg::WARNING, icon::WARNING_ICON_SIZE, assets::warning_color(ui));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.add(egui::Label::new(
            egui::RichText::new(text).weak(),
        ));
    });
}

/// 情報ノートを表示
/// * `ui` - UI
/// * `text` - 情報ノートのテキスト
/// * `space` - スペースの幅
pub(crate) fn info_note(
    ui: &mut egui::Ui,
    text: &str,
    space: Option<f32>,
) {
    ui.horizontal(|ui| {
        if let Some(space) = space {
            ui.add_space(space);
        }

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
