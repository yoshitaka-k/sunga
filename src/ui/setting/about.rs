use crate::{APP_NAME, GITHUB_URL};
use crate::app::UpdateJob;
use crate::ui::{self, setting};
use crate::ui::assets::{self, label, svg, icon};

/// バージョンを表示
/// * `ui` - UI
/// * `update_job` - アップデートジョブ
pub(crate) fn view(ui: &mut egui::Ui, update_job: &mut UpdateJob) {
    // ヘッダーパネルを表示
    setting::header_panel(ui, svg::INFO, label::current().setting_about.heading, Some(update_job));

    ui.separator();

    egui::Frame::default().inner_margin(ui::TOP_PANEL_INNER_MARGIN).show(ui, |ui| {
        // アプリのアイコンを表示
        ui.add(egui::Image::new(assets::APP_ICON).max_height(icon::APP_ICON_SIZE));

        // バージョンを表示
        ui.label(format!("{} v{}", APP_NAME, env!("CARGO_PKG_VERSION")));

        // 説明を表示
        ui.label(env!("CARGO_PKG_DESCRIPTION"));

        ui.label("");

        // ライセンスを表示
        ui.label(format!("License: {}", env!("CARGO_PKG_LICENSE")));

        // リポジトリを表示
        ui.horizontal(|ui| {
            ui.label("Repository:");
            ui.hyperlink_to(env!("CARGO_PKG_REPOSITORY"), GITHUB_URL.replace("{repository}", env!("CARGO_PKG_REPOSITORY")));
        });
    });
}
