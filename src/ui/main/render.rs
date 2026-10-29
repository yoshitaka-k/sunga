use crate::{app, event, file, resize};
use crate::event::{button, click, input, key, open};
use crate::ui::{self, modal, setting};
use crate::ui::assets::{lang, font, svg, icon, sounds};
use crate::ui::main::{self, top, bottom, middle};

/// 描画用のウィジェット
pub struct Render {
    /// アプリケーションの状態
    state: ui::State,

    /// 待機中のアクション
    pending_actions: Vec<event::EventAction>,

    /// ジョブ
    resize_job: resize::ResizeJob,
    update_job: app::UpdateJob,

    /// ステータスカラー
    status_color: ui::StatusColor,

    /// 効果音プレイヤー
    sound: sounds::SoundPlayer,
}

impl Render {
    /// 新しい Render を作成
    /// * `cc` - 作成コンテキスト
    /// * `app` - アプリケーション
    /// * `return` - 新しい Render
    pub fn new(cc: &eframe::CreationContext<'_>, app: app::App) -> Self {
        // フォントと SVG ローダーを追加
        font::install(&cc.egui_ctx);
        svg::install(&cc.egui_ctx);

        // スタイルを設定
        cc.egui_ctx.global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
        });

        // 前回保存した App があれば復元（なければ引数の app を使う）
        let app = cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or(app);

        // アプリケーションの状態を作成
        let state = ui::State {
            app,
            files: file::OpenFiles::new(),
            setting_token: ui::SettingToken::new(),
            updated_token: app::UpdatedToken::new(),
            error_token: ui::ErrorToken::new(),
            scroll_token: ui::ScrollToken::new(),
        };

        Self {
            state,
            pending_actions: Vec::new(),
            resize_job: resize::ResizeJob::new(cc.egui_ctx.clone()),
            update_job: app::UpdateJob::new(cc.egui_ctx.clone()),
            status_color: ui::StatusColor::new(&cc.egui_ctx),
            sound: sounds::SoundPlayer::new(),
        }
    }
}

/// eframe::App の実装
impl eframe::App for Render {
    /// 終了前に App の状態を保存
    /// * `storage` - ストレージ
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state.app);
    }

    /// 描画
    /// * `ui` - UI
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // リサイズ結果を受信
        self.resize_result();

        // 更新結果を取得
        self.update_job.result(&mut self.state.updated_token);

        // 選択された行を解除
        self.state.scroll_token.selected = false;

        // 入力イベントを処理
        self.input_events(ui);

        // ドラッグドロップでファイルを開く
        self.drop_files(ui);

        // 待機中のアクションを処理
        self.process_pending_actions(ui);

        // リサイズを実行
        self.resize_run();

        self.state.files.update_file_info();

        // 行の高さを取得
        let row_height = self.row_height(ui);

        // トップメニューバーを描画
        top::view(
            ui,
            &self.state,
            &mut self.pending_actions,
        );

        // 下部メニューバーを描画
        bottom::view(
            ui,
            &self.state,
            &mut self.pending_actions,
            &self.status_color,
        );

        // 中央パネルを描画
        middle::view(
            ui,
            &self.state,
            &mut self.pending_actions,
            &self.status_color,
            row_height,
        );

        // 設定ウィンドウを描画している場合は描画しない
        if self.state.setting_token.open {
            setting::view(
                ui,
                &mut self.state,
                &mut self.pending_actions,
            );
        }

        // エラーメッセージを描画
        if self.state.error_token.open {
            modal::error(ui.ctx(), &mut self.state.error_token);
        }
    }
}

/// private methods
impl Render {
    /// 行の高さを取得
    /// * `ui` - UI
    /// * `return` - 行の高さ
    fn row_height(&self, ui: &egui::Ui) -> f32 {
        ui.text_style_height(&egui::TextStyle::Body).max(icon::LIST_ICON_SIZE) + main::SEPARATOR_HEIGHT
    }

    /// 入力イベントを処理
    /// * `ui` - UI
    fn input_events(&mut self, ui: &mut egui::Ui) {
        // Command + O キーが押されたらファイルを開く
        input::command_open(ui, &mut self.pending_actions);

        // Command + Comma キーが押されたら設定ウィンドウを開く
        input::command_comma(ui, &mut self.pending_actions);

        // 上矢印キーが押されたら選択された行を上に移動する
        input::up(ui, &self.state.files, &mut self.pending_actions);

        // 下矢印キーが押されたら選択された行を下に移動する
        input::down(ui, &self.state.files, &mut self.pending_actions);

        // エンターキーが押されたら選択された行を開く
        input::enter(ui, &self.state.files, &mut self.pending_actions);

        // スペースキーが押されたら選択された行を開く
        input::space(ui, &self.state.files, &mut self.pending_actions);

        // バックスペースキーが押されたら選択された行をキャンセル
        input::backspace(ui, &mut self.pending_actions);
    }

    /// ドロップされたファイルを処理
    /// * `ui` - UI
    fn drop_files(&mut self, ui: &egui::Ui) {
        let Some(result) = open::drop(ui, &self.state.app, &mut self.state.files) else { return; };

        self.state.error_token.reset();

        // ファイルを開く結果を処理
        match result {
            Ok(true) => (),
            Ok(false) => {
                eprintln!("open failed");
                // 一括追加したときモーダルでると煩わしいので一旦コメントアウト
                // self.error_token.show(error::SungaError::open_failed());
            },
            Err(e) => {
                eprintln!("open error: {}", e);
                self.state.error_token.show(e);
            }
        }

        // ウィンドウを前面に持ってくる
        ui.ctx().send_viewport_cmd_to(
            egui::ViewportId::ROOT,
            egui::ViewportCommand::Focus,
        );
    }

    /// リサイズを実行
    fn resize_run(&mut self) {
        // リサイズを実行
        self.resize_job.run(&self.state.app, &mut self.state.files);
    }

    /// リサイズ結果を処理
    fn resize_result(&mut self) {
        // 待機中かリサイズ中かどうかをチェック
        let was_busy = self.state.files.has_standby() || self.state.files.has_resizing();

        // リサイズ結果を受信
        self.resize_job.result(&mut self.state.files);

        // 最適化後か最適化中が終わっているかどうかをチェック
        let now_idle = !self.state.files.has_standby() && !self.state.files.has_resizing();

        // 効果音を再生
        if *self.state.app.play_sound() && was_busy && now_idle {
            // 効果音の音量を設定
            self.sound.set_volume(*self.state.app.sound_volume());

            if self.state.files.has_error() {
                self.sound.play_alert();
            } else {
                self.sound.play_completed();
            }
        }

    }

    /// 待機中のアクションを処理
    /// * `pending_actions` - 待機中のアクション
    fn process_pending_actions(&mut self, ui: &egui::Ui) {
        for action in self.pending_actions.drain(..) {
            let result = match action {
                // ファイルダイアログを表示
                event::EventAction::FilesOpen => {
                    button::files_open(&self.state.app, &mut self.state.files, lang::current().file_dialog.image_files)
                },
                // フォルダダイアログを表示
                event::EventAction::FolderOpen => {
                    button::folders_open(&self.state.app, &mut self.state.files, lang::current().file_dialog.image_files)
                },
                // ファイルをクリア
                event::EventAction::Clear => {
                    button::cancel_and_clear_files(&mut self.state.files, &mut self.resize_job)
                },
                // 設定ウィンドウを表示
                event::EventAction::SettingOpen(pos) => {
                    button::setting_open(&mut self.state.setting_token, pos)
                },
                // ファイルをクリック
                event::EventAction::Click { id } => {
                    click::click(id, &mut self.state.files)
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
                    key::backspace(&mut self.state.files, &mut self.resize_job)
                }
                // 上矢印キーが押されたら選択された行を上に移動する
                event::EventAction::Up { id } => {
                    key::up(id, &mut self.state.files, &mut self.state.scroll_token)
                }
                // 下矢印キーが押されたら選択された行を下に移動する
                event::EventAction::Down { id } => {
                    key::down(id, &mut self.state.files, &mut self.state.scroll_token)
                }
                // 設定ウィンドウを閉じる
                event::EventAction::SettingClose => {
                    self.state.setting_token.open = false;
                    Ok(true)
                }
                // アップデート確認ボタンが押されたらアップデート確認を行う
                event::EventAction::CheckForUpdate => {
                    self.update_job.run();
                    Ok(true)
                }
            };

            if let Err(e) = result {
                eprintln!("Error processing action: {}", e);
                self.state.error_token.reset();
                self.state.error_token.show(e);
            }

            // 再描画を要求
            ui.ctx().request_repaint();
        }
    }
}
