use crate::{event, state, ui, duration_format, lang};
use crate::ui::{assets, main};
use crate::ui::assets::{size, svg};

/// ボトムメニューバーを描画
/// * `ui` - UI
/// * `state` - アプリケーションの状態
/// * `pending_actions` - 待機中のアクション
/// * `status_color` - ステータスカラー
pub(crate) fn view(
    ui: &mut egui::Ui,
    state: &state::State,
    pending_actions: &mut Vec<event::EventAction>,
    status_color: &ui::StatusColor,
) {
    let style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);
    let button_icon_color = assets::button_icon_color(ui);

    // ボトムメニューバーを作成
    egui::Panel::bottom(ui::BOTTOM_PANEL_NAME).frame(style).show(ui, |ui| {
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                let spacing = ui.spacing_mut().item_spacing.x;

                // ジョブの状態を取得
                let status = state.files.job_status();

                // 完了色を取得
                let completed_color = completed_color(state, status_color);

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
                status_value(ui, spacing,
                    lang::current().resize_status.standby,
                    state.files.standby_len(),
                    status_color.standby
                );
                ui.label(",");

                // 処理中
                status_value(ui, spacing,
                    lang::current().resize_status.resizing,
                    state.files.resizing_len(),
                    status_color.resizing
                );
                ui.label(",");

                // 完了
                ui.scope(|ui| {
                    status_value(ui, spacing,
                        lang::current().resize_status.completed,
                        state.files.completed_len(),
                        completed_color
                    );
                    ui.label(",");
                }).response.on_hover_ui(|ui| {
                    // リサイズ済み
                    ui.horizontal(|ui| {
                        status_value(ui, spacing,
                            lang::current().resize_status.resized,
                            state.files.resized_len(),
                            status_color.resized
                        );
                    });

                    // 変更なし
                    ui.horizontal(|ui| {
                        status_value(ui, spacing,
                            lang::current().resize_status.unchanged,
                            state.files.unchanged_len(),
                            status_color.unchanged
                        );
                    });

                    // スキップ
                    ui.horizontal(|ui| {
                        status_value(ui, spacing,
                            lang::current().resize_status.skipped,
                            state.files.skipped_len(),
                            status_color.skipped
                        );
                    });
                });

                // エラー
                status_value(ui, spacing,
                    lang::current().resize_status.error,
                    state.files.error_len(),
                    status_color.error
                );
            },
            |ui| {
                let hover_text = lang::current().main_bottom.clear;
                let clear_button_image = egui::Image::new(svg::CLEAR_ALL).tint(button_icon_color);
                if ui.add_sized(size::ICON_BUTTON_SIZE, egui::Button::image(clear_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Clear);
                }

                ui.separator();
            }
        );
    });
}

/// 完了色を取得
///
/// 優先度: リサイズ済み > 変更なし > スキップ > 完了
///
/// * `state` - アプリケーションの状態
/// * `status_color` - ステータスカラー
/// * `return` - 完了色
fn completed_color(
    state: &state::State,
    status_color: &ui::StatusColor
) -> egui::Color32 {
    if state.files.resized_len() > 0 {
        status_color.resized
    } else if state.files.unchanged_len() > 0 {
        status_color.unchanged
    } else if state.files.skipped_len() > 0 {
        status_color.skipped
    } else {
        status_color.resized
    }
}

/// ステータス値を描画
/// * `ui` - UI
/// * `spacing` - スペース
/// * `label` - ラベル
/// * `value` - 値
/// * `status_color` - ステータスカラー
fn status_value(
    ui: &mut egui::Ui,
    spacing: f32,
    label: &str,
    value: u32,
    status_color: egui::Color32,
) {
    ui.spacing_mut().item_spacing.x = 0.0;
    ui.label(format!("{}: ", label));
    ui::rich_label(ui, &value.to_string(), status_color);
    ui.spacing_mut().item_spacing.x = spacing;
}
