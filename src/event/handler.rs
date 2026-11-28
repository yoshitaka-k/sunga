use crate::{error, event, state, lang};
use crate::event::{button, click, key, open};

/// イベントを処理
/// * `ctx` - コンテキスト
/// * `state` - 状態
/// * `jobs` - ジョブ
/// * `action` - イベントアクション
pub(crate) fn handle(
    ctx: &egui::Context,
    state: &mut state::State,
    job: &mut state::Job,
    action: event::EventAction,
) -> error::Result<()> {
    match action {
        // ファイルダイアログを表示
        event::EventAction::FilesOpen => {
            let label = lang::current().file_dialog.image_files;
            open::file(&job.scan_job, label);
            Ok(())
        },
        // フォルダダイアログを表示
        event::EventAction::FolderOpen => {
            let label = lang::current().file_dialog.image_files;
            open::folder(&job.scan_job, label);
            Ok(())
        },
        // ドロップされたファイルを処理
        event::EventAction::DropOpen(files) => {
            open::drop(&job.scan_job, files);
            // ウィンドウを前面に持ってくる
            ctx.send_viewport_cmd_to(
                egui::ViewportId::ROOT,
                egui::ViewportCommand::Focus,
            );
            Ok(())
        },
        // ファイルをクリア
        event::EventAction::Clear => {
            button::cancel_and_clear_files(&mut state.files, &job.scan_job, &mut job.resize_job)
        },
        // 設定ウィンドウを表示
        event::EventAction::SettingOpen(pos) => {
            button::setting_open(&mut state.setting_token, pos);
            Ok(())
        },
        // 設定ウィンドウを閉じる
        event::EventAction::SettingClose => {
            state.setting_token.open = false;
            Ok(())
        }
        // ファイルをクリック
        event::EventAction::Click { index } => {
            click::click(index, &mut state.files);
            Ok(())
        }
        // ファイルをダブルクリック
        event::EventAction::DoubleClick(path) => {
            click::double_click(&path)
        }
        // 上矢印キーが押されたら選択された行を上に移動する
        event::EventAction::Up => {
            key::up(&mut state.files, &mut state.scroll_token);
            Ok(())
        }
        // 下矢印キーが押されたら選択された行を下に移動する
        event::EventAction::Down => {
            key::down(&mut state.files, &mut state.scroll_token);
            Ok(())
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
            key::backspace(&mut state.files, &mut job.resize_job)
        }
        // アップデート確認ボタンが押されたらアップデート確認を行う
        event::EventAction::CheckForUpdate => {
            job.update_job.run(&mut state.update_token);
            Ok(())
        }
    }
}
