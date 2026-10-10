use crate::{app, event};
use crate::event::button;
use crate::ui::{self, setting};
use crate::ui::assets::{font, svg};
use crate::ui::main::{top, bottom, middle};

/// 描画用のウィジェット
pub struct Render {
    app: app::App,
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
        // 待機中のアクションを処理
        self.process_pending_actions(ui);

        // トップメニューバーを描画
        top::view(ui, &mut self.pending_actions);
        // 下部メニューバーを描画
        bottom::view(ui, &mut self.pending_actions);
        // 中央パネルを描画
        middle::view(ui);

        // 設定ウィンドウを描画している場合は描画しない
        if self.setting_token.open {
            setting::view(ui, &mut self.setting_token);
        }
    }
}

/// private methods
impl Render {
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
