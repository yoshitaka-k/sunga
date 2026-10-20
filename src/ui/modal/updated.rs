use crate::app::{UpdateCheck, UpdatedToken};
use crate::ui::{self, modal};
use crate::ui::assets::{icon, label, svg};

/// 更新モーダルを表示
/// * `ctx` - コンテキスト
/// * `updated_token` - 更新モーダルを表示するためのトークン
pub(crate) fn updated(ctx: &egui::Context, updated_token: &mut UpdatedToken) {
    let Some(check) = updated_token.check.clone() else {
        return;
    };

    // モーダルを表示
    let modal = egui::Modal::new(egui::Id::new("modal_updated")).show(ctx, |ui| {
        ui.set_width(modal::MODAL_WINDOW_WIDTH);

        // 見出し部分
        ui.horizontal(|ui| {
            match &check {
                // アップデートがある場合
                UpdateCheck::Available { .. } => {
                    ui.add(modal::heading_icon(svg::UPDATE, icon::MODAL_ICON_SIZE, egui::Color32::GREEN));
                    ui.heading(label::current().modal_updated.available);
                }
                // アップデートが最新の場合
                UpdateCheck::Latest => {
                    ui.add(modal::heading_icon(svg::UPDATE, icon::MODAL_ICON_SIZE, ui.visuals().text_color()));
                    ui.heading(label::current().modal_updated.latest);
                }
                // アップデートが取得できなかった場合
                UpdateCheck::Failed(_) => {
                    ui.add(modal::heading_icon(svg::ERROR, icon::MODAL_ICON_SIZE, egui::Color32::RED));
                    ui.heading(label::current().modal_updated.failed);
                }
            }
        });

        ui.separator();

        ui.add_space(modal::MODAL_WINDOW_SPACING);

        // 内容部分
        match &check {
            // アップデートがある場合
            UpdateCheck::Available { version, url } => {
                egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui::add_label(ui, &format!("{}{}", label::current().modal_updated.new_version, version), modal::MODAL_LABEL_WIDTH);
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("{}{}", label::current().modal_updated.current_version, env!("CARGO_PKG_VERSION"))).weak(),
                    ));
                    ui.add_space(modal::MODAL_WINDOW_SPACING);
                });

                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(label::current().modal_updated.download).clicked() {
                        ui.ctx().open_url(egui::OpenUrl::new_tab(url));
                    }
                });
            }
            // アップデートが最新の場合
            UpdateCheck::Latest => {
                egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui.label(format!("{}{}", label::current().modal_updated.current_version, env!("CARGO_PKG_VERSION")));
                    ui.add_space(modal::MODAL_WINDOW_SPACING);
                });
            }
            // アップデートが取得できなかった場合
            UpdateCheck::Failed(e) => {
                egui::Frame::default().inner_margin(ui::PANEL_INNER_MARGIN).show(ui, |ui| {
                    ui.label(e);
                    ui.add_space(modal::MODAL_WINDOW_SPACING);
                });
            }
        }
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        updated_token.open = false;
        updated_token.check = None;
    }
}
