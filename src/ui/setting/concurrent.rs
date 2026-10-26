use crate::app;
use crate::ui::{self, setting};
use crate::ui::assets::label;

/// 並行処理数を表示
/// * `ui` - UI
/// * `app` - アプリ
pub(crate) fn view(
    ui: &mut egui::Ui,
    app: &mut app::App
) {
    // グループヘッダーパネルを表示
    setting::group_header_panel(ui, label::current().setting_concurrent.header_concurrent);

    // フレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            ui::add_label(ui, label::current().setting_concurrent.concurrent, setting::CONCURRENT_LABEL_WIDTH);
            ui.scope(|ui| {
                ui.spacing_mut().slider_width = setting::remaining_slider_width(ui);
                ui.add(egui::Slider::new(app.concurrent_num_mut(), setting::CONCURRENT_NUM_MIN..=setting::CONCURRENT_NUM_MAX));
            });
        });

        setting::warning_note(ui, label::current().setting_concurrent.concurrent_warning_note);
    });
}
