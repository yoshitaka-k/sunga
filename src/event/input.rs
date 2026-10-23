use crate::{app, error, event, file, ui};
use crate::event::{button, drop};

/// ドロップされたファイルを処理する
/// * `ui` - ウィジェットのUI
/// * `app` - アプリケーション
/// * `open_files` - 開いているファイル
/// * `return` - ドロップがなければ `None`、あれば処理結果
pub(crate) fn drop(
    ui: &egui::Ui,
    app: &app::App,
    open_files: &mut file::OpenFiles,
) -> Option<error::Result<bool>> {
    ui.ctx().input(|input| {
        let files = input.raw.dropped_files.clone();
        if files.is_empty() {
            None
        } else {
            Some(drop::files(&files, app, open_files))
        }
    })
}

/// Command + Comma キーが押されたら設定ウィンドウを開く
/// * `ui` - ウィジェットのUI
/// * `setting_token` - 設定ウィンドウトークン
pub(crate) fn command_comma(
    ui: &mut egui::Ui,
    setting_token: &mut ui::SettingToken
) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::Comma)
    }) {
        let _ = button::setting_open(ui, setting_token);
    }
}

/// Command + W キーが押されたら設定ダイアログを閉じる
/// * `ui` - UI
/// * `setting_token` - 設定ダイアログのトークン
pub(crate) fn command_w(
    ui: &mut egui::Ui,
    setting_token: &mut ui::SettingToken,
) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::W)
    }) {
        setting_token.open = false;
    }
}

/// エンターキーが押されたら選択された行を開く
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub(crate) fn enter(
    ui: &mut egui::Ui,
    files: &file::OpenFiles,
    pending_actions: &mut Vec<event::EventAction>
) {
    if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
        if let Some(image) = files.selected_image() {
            pending_actions.push(event::EventAction::Enter(image.reveal_path().clone()));
        }
    }
}

/// バックスペースキーが押されたら選択された行をキャンセル
/// * `ui` - ウィジェットのUI
/// * `pending_actions` - 処理予約
pub(crate) fn backspace(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>
) {
    if ui.input(|input| input.key_pressed(egui::Key::Backspace)) {
        pending_actions.push(event::EventAction::Backspace);
    }
}

/// スペースキーが押されたら選択された行を開く
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub(crate) fn space(
    ui: &mut egui::Ui,
    files: &file::OpenFiles,
    pending_actions: &mut Vec<event::EventAction>
) {
    if ui.input(|input| input.key_pressed(egui::Key::Space)) {
        if let Some(image) = files.selected_image() {
            pending_actions.push(event::EventAction::Space(image.reveal_path().clone()));
        }
    }
}
