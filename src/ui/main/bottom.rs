use crate::{event, file, ui, duration_format};
use crate::ui::main;
use crate::ui::assets::{self, icon, label, svg};

const LEN_SPACING_X: f32 = 4.0;

/// ボトムメニューバーを描画
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    files: &mut file::OpenFiles,
    pending_actions: &mut Vec<event::EventAction>,
    status_color: &ui::StatusColor,
) {
    files.update_file_info();

    let style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);
    let button_icon_color = assets::button_icon_color(ui);

    // ボトムメニューバーを作成
    egui::Panel::bottom(main::BOTTOM_PANEL_NAME).frame(style).show(ui, |ui| {
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                let spacing = ui.spacing_mut().item_spacing.x;

                // ジョブの状態を取得
                let status = files.job_status();

                // 完了色
                // 優先度: リサイズ済み > 変更なし > スキップ > 完了
                let completed_color = if files.resized_len() > 0 {
                    status_color.resized
                } else if files.unchanged_len() > 0 {
                    status_color.unchanged
                } else if files.skipped_len() > 0 {
                    status_color.skipped
                } else {
                    status_color.resized
                };

                // ジョブ状態のアイコン
                main::status_icon(ui, &status, status_color, completed_color).on_hover_ui(|ui| {
                    ui.label(format!(
                        "Average duration: {}",
                        duration_format(files.average_duration()),
                    ));
                });

                ui.separator();

                // 未処理
                ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                ui::rich_label(ui, &files.standby_len().to_string(), status_color.standby);
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(format!("{},", label::current().main_bottom.standby));

                // 処理中
                ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                ui::rich_label(ui, &files.resizing_len().to_string(), status_color.resizing);
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(format!("{},", label::current().main_bottom.resizing));

                // 完了
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                    ui::rich_label(ui, &files.completed_len().to_string(), completed_color);
                    ui.spacing_mut().item_spacing.x = spacing;
                    ui.label(format!("{},", label::current().main_bottom.completed));
                }).response.on_hover_ui(|ui| {
                    // リサイズ済み
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                        ui::rich_label(ui, &files.resized_len().to_string(), status_color.resized);
                        ui.spacing_mut().item_spacing.x = spacing;
                        ui.label(label::current().main_bottom.resized);
                    });

                    // 変更なし
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                        ui::rich_label(ui, &files.unchanged_len().to_string(), status_color.unchanged);
                        ui.spacing_mut().item_spacing.x = spacing;
                        ui.label(label::current().main_bottom.unchanged);
                    });

                    // スキップ
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                        ui::rich_label(ui, &files.skipped_len().to_string(), status_color.skipped);
                        ui.spacing_mut().item_spacing.x = spacing;
                        ui.label(label::current().main_bottom.skipped);
                    });
                });

                // エラー
                ui.spacing_mut().item_spacing.x = LEN_SPACING_X;
                ui::rich_label(ui, &files.error_len().to_string(), status_color.error);
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(label::current().main_bottom.error);
            },
            |ui| {
                let hover_text = label::current().main_bottom.clear;
                let clear_button_image = egui::Image::new(svg::CLEAR_ALL).tint(button_icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(clear_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Clear);
                    ui.ctx().request_repaint();
                }

                ui.separator();
            }
        );
    });
}
