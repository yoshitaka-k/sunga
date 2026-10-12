// Clippy と Rust 2018 の慣用から外れた箇所を警告する
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod event;
mod file;
mod ui;
mod error;
mod resize;

pub use app::App;
pub use ui::Render;

/// アプリケーション名
pub const APP_NAME: &str = "ReSizer";

/// GitHub リポジトリ URL
pub const GITHUB_URL: &str = "https://github.com/{repository}";

/// アップデート確認リクエスト URL
pub const REQUEST_URL: &str = "https://api.github.com/repos/{repository}/releases/latest";
