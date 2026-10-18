use crate::{app, resize};
use crate::ui::{self, setting};
use crate::ui::assets::label;

/// リサイズ設定を表示
/// * `ui` - UI
/// * `app` - アプリケーション
pub(crate) fn view(
    ui: &mut egui::Ui,
    app: &mut app::App,
) {
    // グループヘッダーパネルを表示
    ui.horizontal(|ui| {
        ui.add_space(setting::RESIZE_INDENT_SPACING);
        ui.radio_value(app.is_scale_mut(), false, label::current().setting_resize.header_sized);
    });

    // グループフレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.add_enabled_ui(!*app.is_scale(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, label::current().setting_resize.width, setting::RESIZE_LABEL_WIDTH);
                ui.add(
                    egui::DragValue::new(app.width_mut())
                        .range(resize::SIZED_RANGE_MIN..=resize::SIZED_RANGE_MAX)
                        .speed(10)
                        .suffix("px"),
                );

                ui.separator();

                ui::add_label(ui, label::current().setting_resize.height, setting::RESIZE_LABEL_WIDTH);
                ui.add(
                    egui::DragValue::new(app.height_mut())
                        .range(resize::SIZED_RANGE_MIN..=resize::SIZED_RANGE_MAX)
                        .speed(10)
                        .suffix("px"),
                );
            });

            setting::warning_note(ui, label::current().setting_resize.sized_warning_note);
        });
    });

    ui.add_space(setting::ADD_SPACING);

    // グループヘッダーパネルを表示
    ui.horizontal(|ui| {
        ui.add_space(setting::RESIZE_INDENT_SPACING);
        ui.radio_value(app.is_scale_mut(), true, label::current().setting_resize.header_scale);
    });

    // グループフレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.add_enabled_ui(*app.is_scale(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, label::current().setting_resize.scale, setting::RESIZE_LABEL_WIDTH);
                ui.add(
                    egui::DragValue::new(app.scale_mut())
                        .range(resize::SCALE_RANGE_MIN..=resize::SCALE_RANGE_MAX)
                        .speed(10)
                        .suffix("%"),
                );
            });
        });
    });
}
