use crate::app;
use crate::ui::{self, setting};

/// 設定の一般設定を表示する
pub(crate) fn view(
    ui: &mut egui::Ui,
    app: &mut app::App,
) {
    // グループヘッダーパネルを表示
    setting::group_header_panel(ui, "Output path");

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            ui::add_label(ui, "Path:", setting::GENERAL_LABEL_WIDTH);
            ui.add(
                egui::TextEdit::singleline(app.output_path_mut())
                    .desired_width(ui.available_width())
                    .interactive(false),
            );
        });
        ui.horizontal(|ui| {
            ui.add_space(setting::GENERAL_LABEL_WIDTH + ui.spacing().item_spacing.x);
            if ui.button("Browse").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    *app.output_path_mut() = path.to_string_lossy().into_owned();
                }
            }
            if ui.button("Clear").clicked() {
                app.output_path_mut().clear();
            }
        });

        // 出力パスの注意書きを表示
        setting::warning_note(ui, "Leave empty to overwrite the original files.");
    });
}
