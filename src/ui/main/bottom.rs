use crate::{event, ui, duration_format};
use crate::ui::main;
use crate::ui::assets::{self, icon, lang, svg};

const LEN_SPACING_X: f32 = 4.0;

/// ボトムメニューバーを描画
/// * `ui` - UI
/// * `state` - アプリケーションの状態
/// * `pending_actions` - 待機中のアクション
/// * `status_color` - ステータスカラー
pub(crate) fn view(
    ui: &mut egui::Ui,
    state: &ui::State,
    pending_actions: &mut Vec<event::EventAction>,
    status_color: &ui::StatusColor,
) {
    let style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);
    let button_icon_color = assets::button_icon_color(ui);

    // ボトムメニューバーを作成
    egui::Panel::bottom(main::BOTTOM_PANEL_NAME).frame(style).show(ui, |ui| {
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                let spacing = ui.spacing_mut().item_spacing.x;

                // ジョブの状態を取得
                let status = state.files.job_status();

                // 完了色
                // 優先度: リサイズ済み > 変更なし > スキップ > 完了
                let completed_color = if state.files.resized_len() > 0 {
                    status_color.resized
                } else if state.files.unchanged_len() > 0 {
                    status_color.unchanged
                } else if state.files.skipped_len() > 0 {
                    status_color.skipped
                } else {
                    status_color.resized
                };

                // ジョブ状態のアイコン
                main::status_icon(ui, &status, status_color, completed_color, true).on_hover_ui(|ui| {
                    ui.label(format!(
                        "{}: {}",
                        lang::current().main_bottom.average_duration,
                        duration_format(state.files.average_duration()),
                    ));
                });

                ui.separator();

                // 未処理
                ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                ui::rich_label(ui, &state.files.standby_len().to_string(), status_color.standby);
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(format!("{},", lang::current().resize_status.standby));

                // 処理中
                ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                ui::rich_label(ui, &state.files.resizing_len().to_string(), status_color.resizing);
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(format!("{},", lang::current().resize_status.resizing));

                // 完了
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                    ui::rich_label(ui, &state.files.completed_len().to_string(), completed_color);
                    ui.spacing_mut().item_spacing.x = spacing;
                    ui.label(format!("{},", lang::current().resize_status.completed));
                }).response.on_hover_ui(|ui| {
                    // リサイズ済み
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                        ui::rich_label(ui, &state.files.resized_len().to_string(), status_color.resized);
                        ui.spacing_mut().item_spacing.x = spacing;
                        ui.label(lang::current().resize_status.resized);
                    });

                    // 変更なし
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                        ui::rich_label(ui, &state.files.unchanged_len().to_string(), status_color.unchanged);
                        ui.spacing_mut().item_spacing.x = spacing;
                        ui.label(lang::current().resize_status.unchanged);
                    });

                    // スキップ
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                        ui::rich_label(ui, &state.files.skipped_len().to_string(), status_color.skipped);
                        ui.spacing_mut().item_spacing.x = spacing;
                        ui.label(lang::current().resize_status.skipped);
                    });
                });

                // エラー
                ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                ui::rich_label(ui, &state.files.error_len().to_string(), status_color.error);
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(lang::current().resize_status.error);
            },
            |ui| {
                let hover_text = lang::current().main_bottom.clear;
                let clear_button_image = egui::Image::new(svg::CLEAR_ALL).tint(button_icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(clear_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Clear);
                }

                ui.separator();
            }
        );
    });
}
