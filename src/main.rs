// リリースビルド時に Windows でコンソールウィンドウを隠す
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::all, rust_2018_idioms)]

use sunga::{APP_NAME, Config, Render};

/// アプリケーションバージョン
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// ウィンドウのサイズ
const WINDOW_WIDTH: f32 = 580.0;
const WINDOW_HEIGHT: f32 = 240.0;
const MAX_WINDOW_WIDTH: f32 = 800.0;
const MAX_WINDOW_HEIGHT: f32 = 1024.0;

/// メイン関数
fn main() -> eframe::Result {
    env_logger::init();

    // アプリケーションインスタンスを作成
    let config = Config::new();

    // アプリケーションアイコンを読み込む
    let icon = eframe::icon_data::from_png_bytes(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/icon.png"
    )))
    .expect("failed to load app icon");

    // ウィンドウオプションを作成
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(format!("{} v{}", APP_NAME, VERSION))
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_max_inner_size([MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT])
            .with_maximize_button(false)
            .with_icon(icon),
        ..Default::default()
    };

    // アプリケーションを実行
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(Render::new(cc, config))))
    )
}
