use crate::{config, lang};
use crate::ui::{self, setting};

/// 並行処理数を表示
/// * `ui` - UI
/// * `config` - 設定
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &mut config::Config
) {
    // グループヘッダーパネルを表示
    setting::group_header_panel(ui, lang::current().setting_concurrent.header_concurrent);

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_concurrent.concurrent, setting::CONCURRENT_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(config.concurrent_num_mut(), setting::CONCURRENT_NUM_MIN..=setting::CONCURRENT_NUM_MAX));
            });
        });

        setting::warning_note(ui, lang::current().setting_concurrent.concurrent_warning_note);
    });
}
