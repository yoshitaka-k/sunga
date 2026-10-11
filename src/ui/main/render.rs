use crate::{app, error, event, file};
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

        Self {
            app,
            files: file::OpenFiles::new(),
            pending_actions: Vec::new(),
            dialog_token: ui::DialogToken::new(),
            setting_token: ui::SettingToken::new(),
        }
    }
}

/// eframe::App の実装
impl eframe::App for Render {
    /// 描画
    /// * `ui` - UI
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Command + Comma キーが押されたら設定ウィンドウを開く
        input::command_comma(ui, &mut self.setting_token);

        // ファイルを開く
        self.open_dialog();
        self.drop_files(ui);

        // 待機中のアクションを処理
        self.process_pending_actions(ui);

        // トップメニューバーを描画
        top::view(ui, &mut self.pending_actions);
        // 下部メニューバーを描画
        bottom::view(ui, &mut self.pending_actions);
        // 中央パネルを描画
        middle::view(ui, &self.files);

        // 設定ウィンドウを描画している場合は描画しない
        if self.setting_token.open {
            setting::view(ui, &mut self.setting_token);
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
            Ok(true) =>  println!("success"),
            Ok(false) => println!("false"),
            Err(e) => println!("error: {}", e),
        }
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
                    button::clear_files(ui, &mut self.files);
                },
                event::EventAction::SettingOpen => {
                    button::setting_open(ui, &mut self.setting_token);
                },
            }
        }
    }
}
