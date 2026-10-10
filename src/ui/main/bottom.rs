use crate::{event, ui};
use crate::ui::assets::{self, icon, label, svg};

const PANEL_NAME: &str = "bottom_menubar";

/// ボトムメニューバーを描画
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    let style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);
    let button_icon_color = assets::button_icon_color(ui);

    // ボトムメニューバーを作成
    egui::Panel::bottom(PANEL_NAME).frame(style).show(ui, |ui| {
        egui::Sides::new().shrink_left().truncate().show(ui,
            |ui| {
                ui.label("Top left");
            },
            |ui| {
                let hover_text = label::MainBottom::clear();
                let clear_button_image = egui::Image::new(svg::CLEAR_ALL).tint(button_icon_color);
                if ui.add_sized(icon::ICON_BUTTON_SIZE, egui::Button::image(clear_button_image))
                    .on_hover_text(hover_text).clicked()
                {
                    pending_actions.push(event::EventAction::Clear);
                }
            }
        );
    });
}
