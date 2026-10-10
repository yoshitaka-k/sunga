use crate::{event, ui};
use crate::ui::assets::{self, icon, label, svg};

const PANEL_NAME: &str = "top_menubar";

/// トップメニューバーを描画
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    let style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);
    let button_icon_color = assets::button_icon_color(ui);

    // トップメニューバーを作成
    egui::Panel::top(PANEL_NAME).frame(style).show(ui, |ui| {
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                ui.label("Top left");
            },
            |ui| {
                // 設定ボタン
                let hover_text = label::MainTop::settings();
                let settings_button_image = egui::Image::new(svg::SETTINGS).tint(button_icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(settings_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::SettingOpen);
                }

                // フォルダダイアログを開くボタン
                let hover_text = label::MainTop::folder_open();
                let folder_open_button_image = egui::Image::new(svg::FOLDER_OPEN).tint(button_icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(folder_open_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::FolderOpen);
                }

                // ファイルダイアログを開くボタン
                let hover_text = label::MainTop::files_open();
                let file_open_button_image = egui::Image::new(svg::FILE_OPEN).tint(button_icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(file_open_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::FilesOpen);
                }
            },
        );
    });
}
