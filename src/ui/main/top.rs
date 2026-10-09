use crate::ui;

const PANEL_NAME: &str = "top_menubar";

/// トップメニューバーを描画
/// * `ui` - UI
pub(crate) fn view(ui: &mut egui::Ui) {
    let style = ui::panel_style(ui, ui::TOP_PANEL_INNER_MARGIN);

    // トップメニューバーを作成
    egui::Panel::top(PANEL_NAME).frame(style).show(ui, |ui| {
        ui.label("Top Menubar");
    });
}
