mod error;
mod updated;
pub(crate) use error::error;
pub(crate) use updated::updated;

// モーダルのウィンドウの幅
const MODAL_WINDOW_WIDTH: f32 = 300.0;

// モーダルのウィンドウのスペース
const MODAL_WINDOW_SPACING: f32 = 8.0;

// モーダルのラベルの幅
const MODAL_LABEL_WIDTH: f32 = 130.0;

/// 見出し横のアイコン
/// * `icon` - アイコン
/// * `size` - アイコンのサイズ
/// * `tint` - アイコンの色
/// * `return` - アイコンウィジェット
fn heading_icon(
    icon: egui::ImageSource<'static>,
    size: f32,
    tint: egui::Color32
) -> egui::Image<'static> {
    // horizontal の行高 (interact_size.y = 18) に丸められるため、実サイズ (size) を指定する
    egui::Image::new(icon).fit_to_exact_size(egui::vec2(size, size)).tint(tint)
}
