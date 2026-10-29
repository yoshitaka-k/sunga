// Clippy と Rust 2018 の慣用から外れた箇所を警告する
#![warn(clippy::all, rust_2018_idioms)]

mod config;
mod state;
mod lang;
mod update;
mod event;
mod file;
mod ui;
mod error;
mod resize;

pub use config::Config;
pub use ui::Render;

/// アプリケーション名
pub const APP_NAME: &str = "Sunga";

/// GitHub リポジトリ URL
pub const GITHUB_URL: &str = "https://github.com/{repository}";

/// アップデート確認リクエスト URL
pub const REQUEST_URL: &str = "https://api.github.com/repos/{repository}/releases/latest";

/// 処理時間をフォーマット
pub fn duration_format(duration: u64) -> String {
    if duration < 1000 {
        format!("{:.2} ms", duration as f64)
    } else if duration < 60 * 1000 {
        format!("{:.2} s", duration as f64 / 1000.0)
    } else if duration < 60 * 60 * 1000 {
        format!("{:.2} m", duration as f64 / 60.0 / 1000.0)
    } else {
        format!("{:.2} h", duration as f64 / 60.0 / 60.0 / 1000.0)
    }
}
