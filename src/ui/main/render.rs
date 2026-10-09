use crate::app;
use crate::ui::main::{top, bottom, middle};

/// 描画用のウィジェット
pub struct Render {
    app: app::App,
}

impl Render {
    pub fn new(_cc: &eframe::CreationContext<'_>, app: app::App) -> Self {
        Self { app }
    }
}

/// eframe::App の実装
impl eframe::App for Render {

    /// 描画
    /// * `ui` - UI
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        top::view(ui);
        bottom::view(ui);
        middle::view(ui);
    }
}
