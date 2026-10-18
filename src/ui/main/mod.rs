pub(crate) mod render;
mod top;
mod bottom;
mod middle;

const TOP_PANEL_NAME: &str = "top_menubar";
const BOTTOM_PANEL_NAME: &str = "bottom_menubar";
const SEPARATOR_HEIGHT: f32 = 4.0;
const LIST_ICON_INDENT: f32 = 2.0;

/// アイコンウィジェットを作成
/// * `icon` - アイコン
/// * `size` - アイコンのサイズ
/// * `color` - アイコンの色
/// * `return` - アイコンウィジェット
fn icon_widget(icon: egui::ImageSource<'static>, size: f32, color: egui::Color32) -> impl egui::Widget {
    egui::Image::new(icon).max_height(size).tint(color)
}
