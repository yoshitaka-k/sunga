use crate::app;
use crate::ui::{self, setting};
use crate::ui::assets::label;

/// 設定の一般設定を表示する
pub(crate) fn view(
    ui: &mut egui::Ui,
    app: &mut app::App,
) {
    // グループヘッダーパネルを表示
    setting::group_header_panel(ui, label::SettingGeneral::header_output());

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            ui::add_label(ui, label::SettingGeneral::path(), setting::GENERAL_LABEL_WIDTH);
            ui.add(
                egui::TextEdit::singleline(app.output_path_mut())
                    .desired_width(ui.available_width())
                    .interactive(false),
            );
        });
        ui.horizontal(|ui| {
            ui.add_space(setting::GENERAL_LABEL_WIDTH + ui.spacing().item_spacing.x);
            if ui.button(label::SettingGeneral::browse()).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    *app.output_path_mut() = path.to_string_lossy().into_owned();
                }
            }
            if ui.button(label::SettingGeneral::clear()).clicked() {
                app.output_path_mut().clear();
            }
        });

        // 出力パスの注意書きを表示
        setting::warning_note(ui, label::SettingGeneral::output_warning_note());

        ui.add_space(setting::ADD_SPACING);
        ui.separator();
        ui.add_space(setting::ADD_SPACING);

        // 同じパスはスキップ
        ui.horizontal(|ui| {
            ui::add_label(ui, label::SettingGeneral::skip_same_path(), setting::GENERAL_LABEL_WIDTH);
            ui.radio_value(app.skip_same_path_mut(), true, label::SettingGeneral::skip_same_path_option());
            ui.radio_value(app.skip_same_path_mut(), false, label::SettingGeneral::dont_skip_same_path_option());
        });

        // 同じパスはスキップの注意書きを表示
        setting::warning_note(ui, label::SettingGeneral::skip_same_path_warning_note());
    });
}
