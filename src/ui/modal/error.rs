use crate::{state, lang};
use crate::ui::{self, modal};
use crate::ui::assets::{icon, svg};

/// エラーモーダルを表示
/// * `show_modal` - モーダルを表示するかどうか
/// * `ctx` - コンテキスト
/// * `error_token` - エラーモーダルを表示するためのトークン
pub(crate) fn error(ctx: &egui::Context, error_token: &mut state::ErrorToken) {
    if error_token.value.is_none() {
        return;
    }

    // モーダルを表示
    let modal = egui::Modal::new(egui::Id::new("modal_error")).show(ctx, |ui| {
        ui.set_width(modal::MODAL_WINDOW_WIDTH);

        // 見出し部分
        ui.horizontal(|ui| {
            ui.add(modal::heading_icon(svg::ERROR, icon::MODAL_ICON_SIZE, egui::Color32::RED));
            ui.heading(lang::current().modal_error.heading);
        });

        ui.separator();

        ui.add_space(modal::MODAL_WINDOW_SPACING);

        // エラー内容を表示
        if let Some(error) = &error_token.value {
            egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                ui.label(error.to_string());

                // エラーコードを表示
                let code = error.code();
                ui.add_space(modal::MODAL_WINDOW_SPACING);
                ui.label(egui::RichText::new(format!("Code: {}", code)).weak());
            });
        }

        ui.add_space(modal::MODAL_WINDOW_SPACING);
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        error_token.dismiss();
    }
}
