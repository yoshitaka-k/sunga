use crate::{APP_NAME, GITHUB_URL};
use crate::{event, lang, ui, update};
use crate::ui::{assets, setting};
use crate::ui::assets::{size, svg};

/// バージョンを表示
/// * `ui` - UI
/// * `update_token` - 更新モーダルを表示するためのトークン
/// * `pending_actions` - 処理予約
pub(crate) fn view(
    ui: &mut egui::Ui,
    update_token: &update::UpdateToken,
    pending_actions: &mut Vec<event::EventAction>,
) {
    // アプリ情報のヘッダーパネルを表示
    setting::about_header_panel(
        ui,
        svg::INFO,
        lang::current().setting_about.heading,
        update_token,
        pending_actions
    );

    ui.separator();

    egui::Frame::default().inner_margin(ui::TOP_PANEL_INNER_MARGIN).show(ui, |ui| {
        // アプリのアイコンを表示
        ui.add(egui::Image::new(assets::APP_ICON).max_height(size::APP_ICON_SIZE));

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
