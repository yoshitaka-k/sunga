use crate::app;
use crate::event::input;
use crate::ui::assets::label;
use crate::ui::{self, setting, modal};

/// 設定ウィンドウを描画
/// * `ctx` - コンテキスト
pub(crate) fn view(
    ctx: &egui::Context,
    app: &mut app::App,
    setting_token: &mut ui::SettingToken,
    update_job: &mut app::UpdateJob,
    updated_token: &mut app::UpdatedToken,
) {
    let window_id = egui::ViewportId::from_hash_of(setting::SETTING_WINDOW_ID);

    // 設定ウィンドウのオプションを設定
    let mut options = egui::ViewportBuilder::default()
        .with_title(label::current().setting.heading)
        .with_inner_size([setting::WINDOW_WIDTH, setting::WINDOW_HEIGHT])
        .with_maximize_button(false)
        .with_resizable(false);

    // ウィンドウの表示位置を指定
    // take()で、取り出して None にする（ボタン押下時だけ位置更新と前面化）
    if let Some(pos) = setting_token.pos.take() {
        // ウィンドウの位置を指定
        options = options.with_position(pos);

        // タブを初期化
        setting_token.tab = ui::SettingTab::General;

        // 表示していたら、ウィンドウの位置を更新して前面に出す
        ctx.send_viewport_cmd_to(window_id, egui::ViewportCommand::OuterPosition(pos));
        ctx.send_viewport_cmd_to(window_id, egui::ViewportCommand::Focus);
    }

    // 設定ウィンドウを表示
    ctx.show_viewport_immediate(window_id, options, |ui, _class| {
        // Command + W キーが押されたら閉じる
        input::command_w(ui, setting_token);

        // 更新結果を取得
        update_job.result(updated_token);

        // 設定ウィンドウの上部パネルを描画
        setting::top::view(ui, setting_token);

        // 設定ウィンドウの中央パネルを描画
        egui::CentralPanel::default().show(ui, |ui| {
            match setting_token.tab {
                ui::SettingTab::General => setting::general::view(ui, app),
                ui::SettingTab::Concurrent => setting::concurrent::view(ui, app),
                ui::SettingTab::Resize => setting::resize::view(ui, app),
                ui::SettingTab::About => setting::about::view(ui, update_job),
            }
        });

        // ウィンドウの閉じるボタンが押されたら閉じる
        if ui.input(|input| input.viewport().close_requested()) {
            setting_token.open = false;
        }

        // 更新モーダルを表示
        if updated_token.open {
            modal::updated(ui, updated_token);
        }
    });
}
