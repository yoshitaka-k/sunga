use crate::{config, lang, ui};
use crate::ui::setting;

/// 設定の出力設定を表示する
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &mut config::Config,
) {
    // 出力パスのヘッダーパネルを表示
    setting::group_header_panel(ui, lang::current().setting_output.header_output);

    // 出力パスのフレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_output.path, ui::GENERAL_PATH_LABEL_WIDTH);
            ui.add(
                egui::TextEdit::singleline(config.output_path_mut())
                    .desired_width(ui.available_width())
                    .interactive(false),
            );
        });
        ui.horizontal(|ui| {
            ui.add_space(ui::GENERAL_PATH_LABEL_WIDTH + ui.spacing().item_spacing.x);
            if ui.button(lang::current().setting_output.browse).clicked()
                && let Some(path) = rfd::FileDialog::new().pick_folder()
            {
                *config.output_path_mut() = path.to_string_lossy().into_owned();
            }
            if ui.button(lang::current().setting_output.clear).clicked() {
                config.output_path_mut().clear();
            }
        });

        // 出力パスの注意書きを表示
        setting::warning_note(ui, lang::current().setting_output.output_warning_note, None);

        ui.add_space(ui::ADD_SPACING);
        ui.separator();
        ui.add_space(ui::ADD_SPACING);

        // 同じパスはスキップ
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_output.skip_same_path, ui::GENERAL_LABEL_WIDTH);
            ui.radio_value(config.skip_same_path_mut(), true, lang::current().setting_output.skip_same_path_option);
            ui.radio_value(config.skip_same_path_mut(), false, lang::current().setting_output.dont_skip_same_path_option);
        });

        // 同じパスはスキップの注意書きを表示
        setting::warning_note(ui, lang::current().setting_output.skip_same_path_warning_note, None);
    });
}
