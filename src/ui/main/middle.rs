use crate::{event, file};
use crate::ui::{self, main};
use crate::ui::assets::{self, icon, svg, label};

/// ミドルパネルを描画
/// * `ui` - UI
/// * `open_files` - 開いているファイル
/// * `row_height` - 行の高さ
/// * `status_color` - ステータスカラー
pub(crate) fn view(
    ui: &mut egui::Ui,
    open_files: &mut file::OpenFiles,
    row_height: f32,
    status_color: &ui::StatusColor,
    pending_actions: &mut Vec<event::EventAction>,
) {
    egui::CentralPanel::default().show(ui, |ui| {
        // ファイルの総数を取得
        let total_length = open_files.len();

        // ビューポートの高さを取得
        let viewport_height: f32 = ui.available_height();
        let _row_stride: f32 = row_height + ui.spacing().item_spacing.y;
        let _item_spacing: f32 = ui.spacing().item_spacing.y;

        // スクロールエリアを作成
        let scroll_area = egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .max_height(viewport_height);

        let row_clicked: bool = scroll_area.show_rows(
            ui, row_height, total_length,
            |ui, row_range|
        {
            // 行がクリックされたかどうか
            let mut row_clicked: bool = false;

            // 行の幅と行間を取得
            let width = ui.available_width();
            let row_spacing = ui.spacing().item_spacing.y;
            let start = row_range.start;

            // 表示させる範囲のファイルを取得
            let visibles: Vec<_> = open_files.images()
                .get(row_range)
                .unwrap_or(&[])
                .to_vec();

            // ファイルがない場合はアップロードノートを表示
            if visibles.len() == 0 {
                ui.horizontal(|ui| {
                    ui.add(main::icon_widget(svg::UPLOAD_FILE, icon::MIDDLE_LIST_ICON_SIZE, assets::list_empty_color(ui)));
                    ui.label(assets::text_color(label::current().main_middle.upload_note, assets::list_empty_color(ui), None));
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

                ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(row_rect)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                    |ui|
                {
                    ui.horizontal(|ui| {
                        let spacing = ui.spacing().item_spacing.x;

                        ui.add_space(main::LIST_ICON_INDENT);
                        ui.spacing_mut().item_spacing.x = main::LIST_ICON_INDENT;
                        main::status_icon(ui, image.status(), status_color, status_color.resized);
                        ui.spacing_mut().item_spacing.x = spacing;
                        ui.separator();
                        ui.label(image.file_name());
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
            }

            row_clicked
        }).inner;

        // リスト行以外をクリックしたら選択解除
        if ui.input(|i| i.pointer.primary_clicked()) && !row_clicked {
            open_files.set_selected_id(None);
        }
    });
}
