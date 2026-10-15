use crate::{app, error, event, file, resize};
use crate::event::{button, input, open};
use crate::ui::{self, setting};
use crate::ui::assets::{label, font, svg};
use crate::ui::main::{top, bottom, middle};

/// 描画用のウィジェット
pub struct Render {
    app: app::App,
    files: file::OpenFiles,
    pending_actions: Vec<event::EventAction>,
    dialog_token: ui::DialogToken,
    setting_token: ui::SettingToken,
    resize_job: resize::ResizeJob,
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
            resize_job: resize::ResizeJob::new(cc.egui_ctx.clone()),
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
        // Command + Comma キーが押されたら設定ウィンドウを開く
        input::command_comma(ui, &mut self.setting_token);

        // リサイズ結果を受信
        self.resize_result();

        // ファイルを開く
        self.open_dialog();
        self.drop_files(ui);

        // 待機中のアクションを処理
        self.process_pending_actions(ui);

        // リサイズを実行
        self.resize_run();

        // トップメニューバーを描画
        top::view(ui, &mut self.pending_actions);
        // 下部メニューバーを描画
        bottom::view(ui, &mut self.pending_actions);
        // 中央パネルを描画
        middle::view(ui, &self.files);

        // 設定ウィンドウを描画している場合は描画しない
        if self.setting_token.open {
            setting::view(ui, &mut self.app, &mut self.setting_token);
        }
    }
}

/// private methods
impl Render {
    /// ドロップされたファイルを処理
    /// * `ui` - UI
    fn drop_files(&mut self, ui: &egui::Ui) {
        let Some(result) = input::drop(ui, &mut self.files) else { return; };

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
            let result = open::file(label::FileDialog::image_files(), &mut self.files);
            self.open_result(result);
        }

        // フォルダダイアログを表示
        if self.dialog_token.folder {
            self.dialog_token.folder = false;
            let result = open::folder(label::FileDialog::image_files(), &mut self.files);
            self.open_result(result);
        }
    }

    /// ファイルを開く結果を処理
    /// * `result` - ファイルを開く結果
    fn open_result(&mut self, result: error::Result<bool>) {
        // ファイルを開く結果を処理
        match result {
            Ok(true) =>  println!("open success"),
            Ok(false) => println!("open failed"),
            Err(e) => println!("open error: {}", e),
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
                event::EventAction::FilesOpen => {
                    button::files_open(ui, &mut self.dialog_token);
                },
                event::EventAction::FolderOpen => {
                    button::folders_open(ui, &mut self.dialog_token);
                },
                event::EventAction::Clear => {
                    if let Err(e) = button::cancel_and_clear_files(ui, &mut self.files, &mut self.resize_job) {
                        println!("error: {}", e);
                    }
                },
                event::EventAction::SettingOpen => {
                    button::setting_open(ui, &mut self.setting_token);
                },
            }
        }
    }
}
