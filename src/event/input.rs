use crate::{event, file};

/// Command + O キーが押されたらファイルを開く
/// * `ui` - ウィジェットのUI
/// * `pending_actions` - 処理予約
pub(crate) fn command_open(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>
) {
    // macOS では Command + O キーが押されたらフォルダを開く
    #[cfg(target_os = "macos")]
    {
        if ui.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::COMMAND)
            && input.key_pressed(egui::Key::O)
        }) {
            pending_actions.push(event::EventAction::FolderOpen);
        }
    }

    // Windows や Linux では Command + O キーが押されたらファイルを開く
    // また、Command + Shift + O キーが押されたらフォルダを開く
    #[cfg(not(target_os = "macos"))]
    {
        // Command + O キーが押されたらファイルを開く
        if ui.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::COMMAND)
            && input.key_pressed(egui::Key::O)
        }) {
            pending_actions.push(event::EventAction::FilesOpen);
        }

        // Command + Shift + O キーが押されたらフォルダを開く
        if ui.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::COMMAND | egui::Modifiers::SHIFT)
            && input.key_pressed(egui::Key::O)
        }) {
            pending_actions.push(event::EventAction::FolderOpen);
        }
    }
}

/// Command + Comma キーが押されたら設定ウィンドウを開く
/// * `ui` - ウィジェットのUI
/// * `pending_actions` - 処理予約
pub(crate) fn command_comma(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::Comma)
    }) {
        let pos = ui.ctx().input(|input| {
            input.viewport().outer_rect.map(|rect| rect.min)
        });
        pending_actions.push(event::EventAction::SettingOpen(pos));
    }
}

/// Command + W キーが押されたら設定ダイアログを閉じる
/// * `ui` - UI
/// * `pending_actions` - 処理予約
pub(crate) fn command_w(
    ui: &mut egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::W)
    }) {
        pending_actions.push(event::EventAction::SettingClose);
    }
}

/// 上矢印キーが押されたら選択された行を上に移動する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub(crate) fn up(
    ui: &mut egui::Ui,
    files: &file::OpenFiles,
    pending_actions: &mut Vec<event::EventAction>
) {
    if ui.input(|input| input.key_pressed(egui::Key::ArrowUp))
        && let Some(id) = files.selected_id()
    {
        pending_actions.push(event::EventAction::Up { id: *id });
    }
}

/// 下矢印キーが押されたら選択された行を下に移動する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub(crate) fn down(
    ui: &mut egui::Ui,
    files: &file::OpenFiles,
    pending_actions: &mut Vec<event::EventAction>
) {
    if ui.input(|input| input.key_pressed(egui::Key::ArrowDown))
        && let Some(id) = files.selected_id()
    {
        pending_actions.push(event::EventAction::Down { id: *id });
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
    if ui.input(|input| input.key_pressed(egui::Key::Enter))
        && let Some(image) = files.selected_image()
    {
        pending_actions.push(event::EventAction::Enter(image.reveal_path().clone()));
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
    if ui.input(|input| input.key_pressed(egui::Key::Space))
        && let Some(image) = files.selected_image()
    {
        pending_actions.push(event::EventAction::Space(image.reveal_path().clone()));
    }
}
