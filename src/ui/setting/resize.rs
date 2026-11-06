use crate::{config, resize, lang, ui};
use crate::ui::setting;

const FILTER_ID_SALT: &str = "filter_combo_box";

/// リサイズ設定を表示
/// * `ui` - UI
/// * `config` - 設定
pub(crate) fn view(
    ui: &mut egui::Ui,
    config: &mut config::Config,
) {
    // サイズリサイズモードのヘッダーパネルを表示
    ui.horizontal(|ui| {
        ui.add_space(ui::RESIZE_INDENT_SPACING);
        ui.radio_value(config.target_kind_mut(), resize::TargetKind::Size, lang::current().setting_resize.header_sized);

        ui.add_space(ui::RESIZE_INDENT_SPACING);
        ui.radio_value(config.target_kind_mut(), resize::TargetKind::Scale, lang::current().setting_resize.header_scale);
    });

    // サイズリサイズモードのフレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        // サイズリサイズモードのサイズを表示
        ui.add_enabled_ui(config.target_kind().is_size(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, lang::current().setting_resize.width, ui::RESIZE_LABEL_WIDTH);
                ui.add(
                    egui::DragValue::new(config.width_mut())
                        .range(config::SIZED_RANGE_MIN..=config::SIZED_RANGE_MAX)
                        .speed(10)
                        .suffix("px"),
                );

                ui.separator();

                ui::add_label(ui, lang::current().setting_resize.height, ui::RESIZE_LABEL_WIDTH);
                ui.add(
                    egui::DragValue::new(config.height_mut())
                        .range(config::SIZED_RANGE_MIN..=config::SIZED_RANGE_MAX)
                        .speed(10)
                        .suffix("px"),
                );
            });

            setting::warning_note(ui, lang::current().setting_resize.sized_warning_note, None);
        });

        ui.separator();

        // サイズリサイズモードの倍率を表示
        ui.add_enabled_ui(config.target_kind().is_scale(), |ui| {
            ui.horizontal(|ui| {
                ui::add_label(ui, lang::current().setting_resize.scale, ui::RESIZE_LABEL_WIDTH);
                ui.add(
                    egui::DragValue::new(config.scale_mut())
                        .range(config::SCALE_RANGE_MIN..=config::SCALE_RANGE_MAX)
                        .speed(10)
                        .suffix("%"),
                );
            });
        });
    });

    ui.add_space(ui::ADD_SPACING);

    // フィルターのヘッダーパネルを表示
    setting::group_header_panel(ui, lang::current().setting_resize.header_filter);

    // フィルターのフレームを表示
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.take_available_width();

        ui.horizontal(|ui| {
            ui::add_label(ui, lang::current().setting_resize.filter, ui::RESIZE_LABEL_WIDTH);
            egui::ComboBox::from_id_salt(FILTER_ID_SALT)
                .selected_text(config.filter().to_string())
                .show_ui(ui, |ui| {
                    for filter in resize::filter_iter() {
                        ui.selectable_value(config.filter_mut(), filter, filter.to_string());
                    }
                });
        });

        let note = match config.filter() {
            resize::Filter::Lanczos3 => lang::current().resize_filter_note.lanczos3,
            resize::Filter::CatmullRom => lang::current().resize_filter_note.catmull_rom,
            resize::Filter::Triangle => lang::current().resize_filter_note.triangle,
            resize::Filter::Gaussian => lang::current().resize_filter_note.gaussian,
            resize::Filter::Nearest => lang::current().resize_filter_note.nearest,
        };
        setting::info_note(ui, note, Some(ui::RESIZE_LABEL_WIDTH));
    });
}
