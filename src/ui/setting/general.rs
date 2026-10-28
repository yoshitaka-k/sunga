use crate::app;
use crate::ui::{self, setting};
use crate::ui::assets::lang;

/// 設定の一般設定を表示する
pub(crate) fn view(
    ui: &mut egui::Ui,
    app: &mut app::App,
) {
    // グループヘッダーパネルを表示
    setting::group_header_panel(ui, lang::current().setting_general.header_output);

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.path, setting::GENERAL_PATH_LABEL_WIDTH);
            ui.add(
                egui::TextEdit::singleline(app.output_path_mut())
                    .desired_width(ui.available_width())
                    .interactive(false),
            );
        });
        ui.horizontal(|ui| {
            ui.add_space(setting::GENERAL_PATH_LABEL_WIDTH + ui.spacing().item_spacing.x);
            if ui.button(lang::current().setting_general.browse).clicked()
                && let Some(path) = rfd::FileDialog::new().pick_folder()
            {
                *app.output_path_mut() = path.to_string_lossy().into_owned();
            }
            if ui.button(lang::current().setting_general.clear).clicked() {
                app.output_path_mut().clear();
            }
        });

        // 出力パスの注意書きを表示
        setting::warning_note(ui, lang::current().setting_general.output_warning_note);

        ui.add_space(setting::ADD_SPACING);
        ui.separator();
        ui.add_space(setting::ADD_SPACING);

        // 同じパスはスキップ
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.skip_same_path, setting::GENERAL_LABEL_WIDTH);
            ui.radio_value(app.skip_same_path_mut(), true, lang::current().setting_general.skip_same_path_option);
            ui.radio_value(app.skip_same_path_mut(), false, lang::current().setting_general.dont_skip_same_path_option);
        });

        // 同じパスはスキップの注意書きを表示
        setting::warning_note(ui, lang::current().setting_general.skip_same_path_warning_note);
    });

    ui.add_space(setting::ADD_SPACING);

    // グループヘッダーパネルを表示
    setting::group_header_panel(ui, lang::current().setting_general.header_sound);

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        // 効果音を再生選択
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.sound_effects, setting::GENERAL_LABEL_WIDTH);
            ui.radio_value(app.play_sound_mut(), true, lang::current().setting_general.play_sound);
            ui.radio_value(app.play_sound_mut(), false, lang::current().setting_general.dont_play_sound);
        });

        // 効果音の音量の選択
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.sound_volume, setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(app.sound_volume_mut(), setting::SOUND_VOLUME_MIN..=setting::SOUND_VOLUME_MAX));
            });
        });
    });

}
