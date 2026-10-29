use crate::{event, state, ui, lang};
use crate::ui::main;
use crate::ui::assets::{self, icon, svg};

/// トップメニューバーを描画
/// * `ui` - UI
/// * `state` - アプリケーションの状態
/// * `pending_actions` - 待機中のアクション
pub(crate) fn view(
    ui: &mut egui::Ui,
    state: &state::State,
    pending_actions: &mut Vec<event::EventAction>,
) {
    let style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);
    let button_icon_color = assets::button_icon_color(ui);

    // トップメニューバーを作成
    egui::Panel::top(main::TOP_PANEL_NAME).frame(style).show(ui, |ui| {
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                ui.add(main::icon_widget(svg::RESIZE, icon::TOP_ICON_SIZE, ui::icon_color(ui)));
                ui.label(format!("{}: ", lang::current().main_top.header_resize));

                if *state.config.is_scale() {
                    ui.label(format!("{}: {}% ", lang::current().main_top.header_scale, state.config.scale()));
                } else {
                    ui.label(format!("{}: {}x{} pixels", lang::current().main_top.header_sized, state.config.width(), state.config.height()));
                }
            },
            |ui| {
                // 設定ボタン
                let hover_text = lang::current().main_top.settings;
                let settings_button_image = egui::Image::new(svg::SETTINGS).tint(button_icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(settings_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    let pos = ui.ctx().input(|input| {
                        input.viewport().outer_rect.map(|rect| rect.min)
                    });
                    pending_actions.push(event::EventAction::SettingOpen(pos));
                }

                // フォルダダイアログを開くボタン
                #[cfg(target_os = "macos")]
                {
                    let svg_image = egui::Image::new(svg::UPLOAD).tint(button_icon_color);
                    if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(svg_image))
                        .on_hover_text(lang::current().main_top.file_or_folder_open).clicked()
                    {
                        pending_actions.push(event::EventAction::FolderOpen);
                    }
                }
                #[cfg(not(target_os = "macos"))]
                {
                    let svg_image = egui::Image::new(svg::FOLDER_OPEN).tint(button_icon_color);
                    if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(svg_image))
                        .on_hover_text(lang::current().main_top.folder_open).clicked()
                    {
                        pending_actions.push(event::EventAction::FolderOpen);
                    }
                }

                // ファイルダイアログを開くボタン
                // Macではフォルダダイアログでもファイルを開けるため、表示しないようにする
                #[cfg(not(target_os = "macos"))]
                {
                    let file_open_button_image = egui::Image::new(svg::FILE_OPEN).tint(button_icon_color);
                    if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(file_open_button_image))
                        .on_hover_text(lang::current().main_top.files_open).clicked()
                    {
                        pending_actions.push(event::EventAction::FilesOpen);
                    }
                }
            },
        );
    });
}
