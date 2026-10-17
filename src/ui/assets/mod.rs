pub(crate) mod color;
pub(crate) mod font;
pub(crate) mod icon;
pub(crate) mod svg;
pub(crate) mod label;

/// アプリアイコン
pub(crate) const APP_ICON: egui::ImageSource<'static> = svg::bytes_source(
    "bytes://assets/icon.png",
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png")),
);

/// 交互に表示する背景色を取得
/// * `ui` - UI
/// * `return` - 交互に表示する背景色
pub(crate) fn alternate_background_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 5)
    } else {
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 10)
    }
}

/// ラベルと同じ色を返す
/// * `ui` - UI
/// * `return` - アイコンの色
pub(crate) fn icon_color(ui: &egui::Ui) -> egui::Color32 {
    ui.visuals().text_color()
}

/// リストなし時のテキスト色
/// * `ui` - UI
/// * `return` - リストなし時のテキストの色
pub(crate) fn list_empty_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        color::DARK_MODE_LIST_EMPTY_COLOR
    } else {
        color::LIGHT_MODE_LIST_EMPTY_COLOR
    }
}

/// フォントカラーを設定する
/// * `text` - テキスト
/// * `color` - カラー
/// * `size` - 文字サイズ
/// * `return` - テキスト
pub(crate) fn text_color(text: &str, color: egui::Color32, size: Option<f32>) -> egui::RichText {
    let mut rich_text = egui::RichText::new(text).color(color);
    if let Some(size) = size {
        rich_text = rich_text.size(size);
    }
    rich_text
}

/// ボタンアイコンの色
/// * `ui` - UI
/// * `return` - ボタンアイコンの色
pub(crate) fn button_icon_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        color::DARK_MODE_BUTTON_ICON_COLOR
    } else {
        color::LIGHT_MODE_BUTTON_ICON_COLOR
    }
}

/// 警告アイコンの色
/// * `ui` - UI
/// * `return` - 警告アイコンの色
pub(crate) fn warning_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        color::DARK_MODE_WARNING_COLOR
    } else {
        color::LIGHT_MODE_WARNING_COLOR
    }
}
