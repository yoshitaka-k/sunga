pub(crate) mod color;
pub(crate) mod font;
pub(crate) mod icon;
pub(crate) mod svg;
pub(crate) mod label;

/// ボタンアイコンの色
/// * `ui` - UI
/// * `return` - ボタンアイコンの色
pub(crate) fn button_icon_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        color::DARK_MODE_BUTTON_ICON_COLOR
    } else {
        color::LIGHT_MODE_BUTTON_ICON_COLOR
    }
}
