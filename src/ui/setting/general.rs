use crate::{config, lang};
use crate::ui::{self, setting};

/// 設定の一般設定を表示する
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &mut config::Config,
) {
    // グループヘッダーパネルを表示
    setting::group_header_panel(ui, lang::current().setting_general.header_language);

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            for lang in lang::LIST {
                if ui.radio_value(config.language_mut(), lang.code.to_string(), lang.name.to_string()).changed() {
                    lang::set(&lang.code);
                }
            }
        });
    });

    ui.add_space(setting::ADD_SPACING);

    // グループヘッダーパネルを表示
    setting::group_header_panel(ui, lang::current().setting_general.header_theme);

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            if ui.radio_value(config.theme_mut(), egui::Visuals::dark(), lang::current().setting_general.dark_theme).changed() {
                ui.ctx().set_visuals(egui::Visuals::dark());
            }
            if ui.radio_value(config.theme_mut(), egui::Visuals::light(), lang::current().setting_general.light_theme).changed() {
                ui.ctx().set_visuals(egui::Visuals::light());
            }
        });
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
            ui.radio_value(config.play_sound_mut(), true, lang::current().setting_general.play_sound);
            ui.radio_value(config.play_sound_mut(), false, lang::current().setting_general.dont_play_sound);
        });

        // 効果音の音量の選択
        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_general.sound_volume, setting::GENERAL_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(config.sound_volume_mut(), setting::SOUND_VOLUME_MIN..=setting::SOUND_VOLUME_MAX));
            });
        });
    });
}
