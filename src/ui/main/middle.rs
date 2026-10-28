use crate::{event, file, duration_format};
use crate::ui::{self, main};
use crate::ui::assets::{self, icon, svg, lang};

/// ミドルパネルを描画
/// * `ui` - UI
/// * `open_files` - 開いているファイル
/// * `row_height` - 行の高さ
/// * `status_color` - ステータスカラー
/// * `pending_actions` - 待機中のアクション
/// * `scroll_token` - スクロールエリアを表示するためのトークン
pub(crate) fn view(
    ui: &mut egui::Ui,
    open_files: &mut file::OpenFiles,
    row_height: f32,
    status_color: &ui::StatusColor,
    pending_actions: &mut Vec<event::EventAction>,
    scroll_token: &mut ui::ScrollToken,
) {
    egui::CentralPanel::default().show(ui, |ui| {
        // ファイルの総数を取得
        let total_length = open_files.len();

        // ビューポートの高さを取得
        let viewport_height: f32 = ui.available_height();
        let row_stride: f32 = row_height + ui.spacing().item_spacing.y;

        // スクロールエリアを作成
        let scroll_area = egui::ScrollArea::vertical()
            .animated(false)
            .auto_shrink([false, false])
            .max_height(viewport_height);

        // スクロールエリアを表示
        let row_clicked: bool = scroll_area.show_rows(
            ui, row_height, total_length,
            |ui, row_range|
        {
            // 行の幅と行間を取得
            let width = ui.available_width();
            let row_spacing = ui.spacing().item_spacing.y;
            let start = row_range.start;

            // 選択された行がある場合はスクロールエリアの表示位置を調整
            if scroll_token.selected {
                if let Some(index) = open_files.selected_index() {
                    // スクロール位置への矩形を計算・作成
                    let area = ui.max_rect();
                    let top = area.top() + (index as f32 - start as f32) * row_stride;
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(area.left(), top),
                        egui::vec2(area.width(), row_height),
                    );

                    // スクロールエリアの表示位置を調整
                    // ui.scroll_to_rect(rect, None);
                    ui.scroll_to_rect_animation(rect, None, egui::style::ScrollAnimation::none());
                }

                scroll_token.selected = false;
            }

            // 行がクリックされたかどうか
            let mut row_clicked: bool = false;

            // 表示させる範囲のファイルを取得
            let visibles: Vec<_> = open_files.images()
                .get(row_range)
                .unwrap_or(&[])
                .to_vec();

            // ファイルがない場合はアップロードノートを表示
            if visibles.len() == 0 {
                ui.horizontal(|ui| {
                    ui.add(main::icon_widget(svg::UPLOAD_FILE, icon::MIDDLE_LIST_ICON_SIZE, assets::list_empty_color(ui)));
                    ui.label(assets::text_color(lang::current().main_middle.upload_note, assets::list_empty_color(ui), None));
                });
            }

            for (offset, image) in visibles.iter().enumerate() {
                let index = start + offset;

                // 行をクリックできるようにする
                let (row_rect, response) = ui.allocate_exact_size(
                    egui::vec2(width, row_height),
                    egui::Sense::click(),
                );

                // 交互に背景色を表示
                if index % 2 == 0 {
                    ui.painter().rect_filled(row_rect, main::LIST_CORNER_RADIUS, assets::alternate_background_color(ui));
                }

                // 選択されている場合は背景を表示
                if *open_files.selected_id() == Some(*image.id()) {
                    ui.painter().rect_filled(row_rect, main::LIST_CORNER_RADIUS, assets::selected_background_color(ui));
                }

                // ステータスに応じてノートを表示
                let note = match image.status() {
                    file::Status::Standby | file::Status::Resizing => None,
                    file::Status::Resized => Some(egui::RichText::new(duration_format(*image.duration()))),
                    file::Status::Skipped => Some(assets::text_color(lang::current().resize_status.skipped, status_color.skipped, None)),
                    file::Status::Unchanged => Some(assets::text_color(lang::current().resize_status.unchanged, status_color.unchanged, None)),
                    file::Status::Canceled => Some(assets::text_color(lang::current().resize_status.canceled, status_color.canceled, None)),
                    file::Status::Error(e) => Some(assets::text_color(e, status_color.error, None)),
                };

                // アイテムの表示
                ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(row_rect)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                    |ui|
                {
                    ui.horizontal(|ui| {
                        // 折り返さないようにして、溢れた分は省略
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);

                        // デフォルトのスペースを取得
                        let spacing = ui.spacing().item_spacing.x;

                        // ステータスアイコンを表示
                        ui.add_space(main::LIST_ICON_INDENT);
                        ui.spacing_mut().item_spacing.x = main::LIST_ICON_INDENT;
                        main::status_icon(ui, image.status(), status_color, status_color.resized, false)
                            .on_hover_text(image.status().to_string());
                        ui.spacing_mut().item_spacing.x = spacing;

                        ui.separator();

                        // ディレクトリか、ファイルかを表示
                        main::opentype_icon(ui, image);

                        // ファイル名とステータスノートを表示
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if let Some(note) = note {
                                ui.add_space(main::LIST_RIGHT_INDENT);
                                ui.label(note);
                                ui.separator();
                            }
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.set_min_width(ui.available_width() * main::LIST_NAME_MIN_RATIO);
                                ui.add(egui::Label::new(image.file_name().as_ref()).truncate());
                            });
                        });
                    });
                });

                // 最終行以外は行下端に区切り線
                if index + 1 < total_length {
                    ui.painter().hline(
                        row_rect.x_range(),
                        row_rect.bottom() + row_spacing * 0.5,
                        ui.visuals().widgets.noninteractive.bg_stroke,
                    );
                }

                // 行がクリックされた場合は選択されたアイテムを更新
                if response.clicked() {
                    pending_actions.push(event::EventAction::Click { id: *image.id() });
                    row_clicked = true;
                }

                // 行がダブルクリックされた場合はファイルパスを取得
                if response.double_clicked() {
                    pending_actions.push(event::EventAction::DoubleClick(image.reveal_path().clone()));
                }
            }

            row_clicked
        }).inner;

        // リスト行以外をクリックしたら選択解除
        if ui.input(|i| i.pointer.primary_clicked()) && !row_clicked {
            open_files.set_selected_id(None);
        }
    });
}
