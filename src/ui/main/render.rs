use crate::{app, error, event, file, resize};
use crate::event::{button, click, input, open};
use crate::ui::{self, modal, setting};
use crate::ui::assets::{label, font, svg, icon};
use crate::ui::main::{self, top, bottom, middle};

/// 描画用のウィジェット
pub struct Render {
    app: app::App,
    files: file::OpenFiles,
    pending_actions: Vec<event::EventAction>,
    dialog_token: ui::DialogToken,
    setting_token: ui::SettingToken,
    updated_token: app::UpdatedToken,
    error_token: ui::ErrorToken,
    resize_job: resize::ResizeJob,
    update_job: app::UpdateJob,
    status_color: ui::StatusColor,
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

        Self {
            app,
            files: file::OpenFiles::new(),
            pending_actions: Vec::new(),
            dialog_token: ui::DialogToken::new(),
            setting_token: ui::SettingToken::new(),
            updated_token: app::UpdatedToken::new(),
            error_token: ui::ErrorToken::new(),
            resize_job: resize::ResizeJob::new(cc.egui_ctx.clone()),
            update_job: app::UpdateJob::new(cc.egui_ctx.clone()),
            status_color: ui::StatusColor::new(&cc.egui_ctx),
        }
    }
}

/// eframe::App の実装
impl eframe::App for Render {
    /// 終了前に App の状態を保存
    /// * `storage` - ストレージ
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.app);
    }

    /// 描画
    /// * `ui` - UI
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 入力イベントを処理
        self.input_events(ui);

        // リサイズ結果を受信
        self.resize_result();

        // ファイルを開く
        self.open_dialog();
        self.drop_files(ui);

        // 待機中のアクションを処理
        self.process_pending_actions(ui);

        // リサイズを実行
        self.resize_run();

        // 行の高さを取得
        let row_height = self.row_height(ui);

        // トップメニューバーを描画
        top::view(
            ui,
            &self.app,
            &mut self.pending_actions,
        );

        // 下部メニューバーを描画
        bottom::view(
            ui,
            &mut self.files,
            &mut self.pending_actions,
            &self.status_color,
        );

        // 中央パネルを描画
        middle::view(
            ui,
            &mut self.files,
            row_height,
            &self.status_color,
            &mut self.pending_actions,
        );

        // 設定ウィンドウを描画している場合は描画しない
        if self.setting_token.open {
            setting::view(
                ui,
                &mut self.app,
                &mut self.setting_token,
                &mut self.update_job,
                &mut self.updated_token,
            );
        }

        // エラーメッセージを描画
        if self.error_token.open {
            modal::error(ui.ctx(), &mut self.error_token);
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
        // Command + Comma キーが押されたら設定ウィンドウを開く
        input::command_comma(ui, &mut self.setting_token);

        // エンターキーが押されたら選択された行を開く
        input::enter(ui, &self.files, &mut self.pending_actions);
    }

    /// ドロップされたファイルを処理
    /// * `ui` - UI
    fn drop_files(&mut self, ui: &egui::Ui) {
        let Some(result) = input::drop(ui, &mut self.app, &mut self.files) else { return; };

        self.open_result(result);

        // ウィンドウを前面に持ってくる
        ui.ctx().send_viewport_cmd_to(
            egui::ViewportId::ROOT,
            egui::ViewportCommand::Focus,
        );
    }

    /// ファイルダイアログを表示
    fn open_dialog(&mut self) {
        // ファイルダイアログを表示
        if self.dialog_token.file {
            self.dialog_token.file = false;
            let result = open::file(&self.app, label::current().file_dialog.image_files, &mut self.files);
            self.open_result(result);
        }

        // フォルダダイアログを表示
        if self.dialog_token.folder {
            self.dialog_token.folder = false;
            let result = open::folder(&self.app, label::current().file_dialog.image_files, &mut self.files);
            self.open_result(result);
        }
    }

    /// ファイルを開く結果を処理
    /// * `result` - ファイルを開く結果
    fn open_result(&mut self, result: error::Result<bool>) {
        self.error_token.reset();

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
                self.error_token.show(e);
            }
        }
    }

    /// リサイズを実行
    fn resize_run(&mut self) {
        // リサイズを実行
        self.resize_job.run(&self.app, &mut self.files);
    }

    /// リサイズ結果を処理
    fn resize_result(&mut self) {
        // リサイズ結果を受信
        self.resize_job.result(&mut self.files);
    }

    /// 待機中のアクションを処理
    /// * `pending_actions` - 待機中のアクション
    fn process_pending_actions(&mut self, ui: &egui::Ui) {
        for action in self.pending_actions.drain(..) {
            match action {
                // ファイルダイアログを表示
                event::EventAction::FilesOpen => {
                    button::files_open(ui, &mut self.dialog_token);
                },
                // フォルダダイアログを表示
                event::EventAction::FolderOpen => {
                    button::folders_open(ui, &mut self.dialog_token);
                },
                // ファイルをクリア
                event::EventAction::Clear => {
                    if let Err(e) = button::cancel_and_clear_files(ui, &mut self.files, &mut self.resize_job) {
                        println!("error: {}", e);
                        self.error_token.reset();
                        self.error_token.show(e);
                    }
                },
                // 設定ウィンドウを表示
                event::EventAction::SettingOpen => {
                    button::setting_open(ui, &mut self.setting_token);
                },
                // ファイルをクリック
                event::EventAction::Click { id } => {
                    self.files.set_selected_id(Some(id));
                }
                // ファイルをダブルクリック
                event::EventAction::DoubleClick(path) => {
                    if let Err(e) = click::double_click(&path) {
                        eprintln!("Error revealing file: {}", e);
                        self.error_token.reset();
                        self.error_token.show(e);
                    }
                }
                // エンターキーが押されたら選択された行を開く
                event::EventAction::Enter(path) => {
                    if let Err(e) = click::double_click(&path) {
                        eprintln!("Error revealing file: {}", e);
                        self.error_token.reset();
                        self.error_token.show(e);
                    }
                }
            }
        }
    }
}
