/// ミドルパネルを描画
/// * `ui` - UI
pub(crate) fn view(ui: &mut egui::Ui) {
    egui::CentralPanel::default().show(ui, |ui| {
        ui.label("Middle Panel");
    });
}
