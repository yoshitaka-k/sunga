use crate::{error, event, state};
use crate::event::{button, click, key, open};
use crate::lang;

/// イベントを処理
/// * `ctx` - コンテキスト
/// * `state` - 状態
/// * `jobs` - ジョブ
/// * `action` - イベントアクション
pub(crate) fn handle(
    ctx: &egui::Context,
    state: &mut state::State,
    jobs: &mut state::Jobs,
    action: event::EventAction,
) -> error::Result<()> {
    match action {
        // ファイルダイアログを表示
        event::EventAction::FilesOpen => {
            let label = lang::current().file_dialog.image_files;
            button::files_open(&state.config, &mut state.files, label);
            Ok(())
        },
        // フォルダダイアログを表示
        event::EventAction::FolderOpen => {
            let label = lang::current().file_dialog.image_files;
            button::folders_open(&state.config, &mut state.files, label);
            Ok(())
        },
        // ドロップされたファイルを処理
        event::EventAction::DropOpen(files) => {
            open::drop_open(ctx, &state.config, &mut state.files, files);
            Ok(())
        },
        // ファイルをクリア
        event::EventAction::Clear => {
            button::cancel_and_clear_files(&mut state.files, &mut jobs.resize_job)
        },
        // 設定ウィンドウを表示
        event::EventAction::SettingOpen(pos) => {
            button::setting_open(&mut state.setting_token, pos);
            Ok(())
        },
        // ファイルをクリック
        event::EventAction::Click { id } => {
            click::click(id, &mut state.files);
            Ok(())
        }
        // ファイルをダブルクリック
        event::EventAction::DoubleClick(path) => {
            click::double_click(&path)
        }
        // エンターキーが押されたら選択された行を開く
        event::EventAction::Enter(path) => {
            click::double_click(&path)
        }
        // スペースキーが押されたら選択された行を開く
        event::EventAction::Space(path) => {
            key::space(&path)
        }
        // バックスペースキーが押されたら選択された行をキャンセル
        event::EventAction::Backspace => {
            key::backspace(&mut state.files, &mut jobs.resize_job)
        }
        // 上矢印キーが押されたら選択された行を上に移動する
        event::EventAction::Up { id } => {
            key::up(id, &mut state.files, &mut state.scroll_token);
            Ok(())
        }
        // 下矢印キーが押されたら選択された行を下に移動する
        event::EventAction::Down { id } => {
            key::down(id, &mut state.files, &mut state.scroll_token);
            Ok(())
        }
        // 設定ウィンドウを閉じる
        event::EventAction::SettingClose => {
            state.setting_token.open = false;
            Ok(())
        }
        // アップデート確認ボタンが押されたらアップデート確認を行う
        event::EventAction::CheckForUpdate => {
            jobs.update_job.run();
            Ok(())
        }
    }
}
