use crate::ui;

const PANEL_NAME: &str = "bottom_menubar";

/// ボトムメニューバーを描画
/// * `ui` - UI
pub(crate) fn view(ui: &mut egui::Ui) {
    let style = ui::panel_style(ui, ui::BOTTOM_PANEL_INNER_MARGIN);

    // ボトムメニューバーを作成
    egui::Panel::bottom(PANEL_NAME).frame(style).show(ui, |ui| {
        ui.label("Bottom Menubar");
    });
}
