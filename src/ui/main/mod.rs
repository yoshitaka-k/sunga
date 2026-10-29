pub(crate) mod view;
mod top;
mod bottom;
mod middle;

pub(crate) use view::view;

const TOP_PANEL_NAME: &str = "top_menubar";
const BOTTOM_PANEL_NAME: &str = "bottom_menubar";
const SEPARATOR_HEIGHT: f32 = 4.0;
const LIST_ICON_INDENT: f32 = 2.0;
const LIST_CORNER_RADIUS: f32 = 4.0;
const LIST_NAME_MIN_RATIO: f32 = 1.0 / 3.3;
const LIST_RIGHT_INDENT: f32 = 10.0;

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

/// スピナーウィジェットを作成
/// * `size` - スピナーのサイズ
/// * `color` - スピナーの色
/// * `return` - スピナーウィジェット
pub(crate) fn spinner_widget(size: f32, color: egui::Color32) -> impl egui::Widget {
    egui::Spinner::new().size(size).color(color)
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
    completed_color: egui::Color32,
    is_spinner: bool,
) -> egui::Response {
    match status {
        file::Status::Standby => {
            ui.add(icon_widget(svg::CIRCLE, icon::STATUS_ICON_SIZE, status_color.standby))
        },
        file::Status::Resizing => {
            if is_spinner {
                ui.add(spinner_widget(icon::STATUS_ICON_SIZE, status_color.resizing))
            } else {
                ui.add(icon_widget(svg::AUTORENEW, icon::STATUS_ICON_SIZE, status_color.resizing))
            }
        },
        file::Status::Resized => {
            ui.add(icon_widget(svg::CHECK, icon::STATUS_ICON_SIZE, completed_color))
        },
        file::Status::Unchanged => {
            ui.add(icon_widget(svg::CHECK_INDETERMINATE_SMALL, icon::STATUS_ICON_SIZE, status_color.unchanged))
        },
        file::Status::Skipped => {
            ui.add(icon_widget(svg::CHECK_INDETERMINATE_SMALL, icon::STATUS_ICON_SIZE, status_color.skipped))
        },
        file::Status::Canceled => {
            ui.add(icon_widget(svg::CANCEL, icon::STATUS_ICON_SIZE, status_color.canceled))
        },
        file::Status::Error(_) => {
            ui.add(icon_widget(svg::ERROR, icon::STATUS_ICON_SIZE, status_color.error))
        },
    }
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
    let icon = if *path.is_in_folder() {
        svg::FOLDER
    } else {
        svg::PHOTO
    };
    let icon_color = ui::icon_color(ui);
    let hover_text = path.relative_path().display().to_string();

    ui.add(icon_widget(icon, icon::LIST_ICON_SIZE, icon_color)).on_hover_text(hover_text)
}
