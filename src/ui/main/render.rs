use crate::{app, error, event, file};
use crate::event::{button, input};
use crate::ui::{self, setting};
use crate::ui::assets::{font, svg};
use crate::ui::main::{top, bottom, middle};

/// 描画用のウィジェット
pub struct Render {
    app: app::App,
    files: file::OpenFiles,
    pending_actions: Vec<event::EventAction>,
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
        // ファイルを開く
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
                event::EventAction::SettingOpen => {
                    button::setting_open(ui, &mut self.setting_token);
                },
                _ => (),
            }
        }
    }
}
