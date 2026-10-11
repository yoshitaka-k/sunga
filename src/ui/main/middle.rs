use crate::file;

/// ミドルパネルを描画
/// * `ui` - UI
/// * `open_files` - 開いているファイル
pub(crate) fn view(
    ui: &mut egui::Ui,
    open_files: &file::OpenFiles,
) {
    egui::CentralPanel::default().show(ui, |ui| {
        for image in open_files.images().iter() {
            ui.label(image.path().to_string_lossy());
        }
    });
}
