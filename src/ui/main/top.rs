use crate::{event, ui};
use crate::ui::main;
use crate::ui::assets::{self, icon, label, svg};

/// トップメニューバーを描画
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    let style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);
    let button_icon_color = assets::button_icon_color(ui);

    // トップメニューバーを作成
    egui::Panel::top(main::TOP_PANEL_NAME).frame(style).show(ui, |ui| {
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                ui.label("Top left");
            },
            |ui| {
                // 設定ボタン
                let hover_text = label::current().main_top.settings;
                let settings_button_image = egui::Image::new(svg::SETTINGS).tint(button_icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(settings_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::SettingOpen);
                    ui.ctx().request_repaint();
                }

                // フォルダダイアログを開くボタン
                let hover_text = label::current().main_top.folder_open;
                let folder_open_button_image = egui::Image::new(svg::FOLDER_OPEN).tint(button_icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(folder_open_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::FolderOpen);
                    ui.ctx().request_repaint();
                }

                // ファイルダイアログを開くボタン
                let hover_text = label::current().main_top.files_open;
                let file_open_button_image = egui::Image::new(svg::FILE_OPEN).tint(button_icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(file_open_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::FilesOpen);
                    ui.ctx().request_repaint();
                }
            },
        );
    });
}
