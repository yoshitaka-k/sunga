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

            let label = format!("{}: {} ({})",
                image.id(),
                image.file_name(),
                image.status().to_string(),
            );
            ui.label(label);
        }
    });
}
