use crate::{app, event};
use crate::ui::assets::{font, svg};
use crate::ui::main::{top, bottom, middle};

/// 描画用のウィジェット
pub struct Render {
    app: app::App,
    pending_actions: Vec<event::EventAction>,
}

impl Render {
    pub fn new(cc: &eframe::CreationContext<'_>, app: app::App) -> Self {
        // フォントと SVG ローダーを追加
        font::install(&cc.egui_ctx);
        svg::install(&cc.egui_ctx);

        Self {
            app,
            pending_actions: Vec::new(),
        }
    }
}

/// eframe::App の実装
impl eframe::App for Render {

    /// 描画
    /// * `ui` - UI
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        top::view(ui, &mut self.pending_actions);
        bottom::view(ui, &mut self.pending_actions);
        middle::view(ui);
    }
}
