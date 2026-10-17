use crate::file;
use crate::ui::main;
use crate::ui::assets::{self, icon, svg, label};

/// ミドルパネルを描画
/// * `ui` - UI
/// * `open_files` - 開いているファイル
pub(crate) fn view(
    ui: &mut egui::Ui,
    open_files: &file::OpenFiles,
    row_height: f32,
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

        let _row_clicked = scroll_area.show_rows(
            ui, row_height, total_length,
            |ui, row_range|
        {
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
                let (row_rect, _response) = ui.allocate_exact_size(
                    egui::vec2(width, row_height),
                    egui::Sense::click(),
                );

                // 交互に背景色を表示
                if index % 2 == 0 {
                    ui.painter().rect_filled(row_rect, 1.0, assets::alternate_background_color(ui));
                }

                let label = format!("{}: {} ({})",
                    image.id(),
                    image.file_name(),
                    image.status().to_string(),
                );

                ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(row_rect)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                    |ui|
                {
                    ui.label(label);
                });

                // 最終行以外は行下端に区切り線
                if index + 1 < total_length {
                    ui.painter().hline(
                        row_rect.x_range(),
                        row_rect.bottom() + row_spacing * 0.5,
                        ui.visuals().widgets.noninteractive.bg_stroke,
                    );
                }
            }
        });
    });
}
