pub(crate) mod render;
mod top;
mod bottom;
mod middle;

const TOP_PANEL_NAME: &str = "top_menubar";
const BOTTOM_PANEL_NAME: &str = "bottom_menubar";
const SEPARATOR_HEIGHT: f32 = 4.0;
const LIST_ICON_INDENT: f32 = 2.0;
const LIST_CORNER_RADIUS: f32 = 4.0;

use crate::{file, ui};
use crate::ui::assets::{icon, svg};

/// アイコンウィジェットを作成
/// * `icon` - アイコン
/// * `size` - アイコンのサイズ
/// * `color` - アイコンの色
/// * `return` - アイコンウィジェット
fn icon_widget(icon: egui::ImageSource<'static>, size: f32, color: egui::Color32) -> impl egui::Widget {
    egui::Image::new(icon).max_height(size).tint(color)
}

/// ステータスアイコンを描画
/// * `ui` - UI
/// * `status` - ステータス
/// * `status_color` - ステータスカラー
/// * `completed_color` - 完了色
/// * `return` - ステータスアイコン
fn status_icon(
    ui: &mut egui::Ui,
    status: &file::Status,
    status_color: &ui::StatusColor,
    completed_color: egui::Color32
) -> egui::Response {
    let widget = match status {
       file::Status::Standby => {
           icon_widget(svg::CIRCLE, icon::STATUS_ICON_SIZE, status_color.standby)
       },
       file::Status::Resizing => {
           icon_widget(svg::AUTORENEW, icon::STATUS_ICON_SIZE, status_color.resizing)
       },
       file::Status::Resized => {
           icon_widget(svg::CHECK, icon::STATUS_ICON_SIZE, completed_color)
       },
       file::Status::Unchanged => {
           icon_widget(svg::CHECK_INDETERMINATE_SMALL, icon::STATUS_ICON_SIZE, status_color.unchanged)
       },
       file::Status::Skipped => {
           icon_widget(svg::CHECK_INDETERMINATE_SMALL, icon::STATUS_ICON_SIZE, status_color.skipped)
       },
       file::Status::Canceled => {
           icon_widget(svg::CANCEL, icon::STATUS_ICON_SIZE, status_color.canceled)
       },
       file::Status::Error(_) => {
           icon_widget(svg::ERROR, icon::STATUS_ICON_SIZE, status_color.error)
       },
   };

   ui.add(widget)
}

/// ディレクトリか、ファイルかを表示
/// * `ui` - UI
/// * `pad` - パディング
/// * `path` - パス
/// * `return` - OpenTypeアイコンのレスポンス
pub(crate) fn opentype_icon(
    ui: &mut egui::Ui,
    path: &file::Image,
) -> egui::Response {
    let icon_color = ui::icon_color(ui);
    if *path.is_in_folder() {
        let hover_text = path.relative_path().display().to_string();
        ui.add(icon_widget(svg::FOLDER, icon::LIST_ICON_SIZE, icon_color)).on_hover_text(hover_text)
    } else {
        ui.add(icon_widget(svg::PHOTO, icon::LIST_ICON_SIZE, icon_color))
    }
}
