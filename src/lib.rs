// Clippy と Rust 2018 の慣用から外れた箇所を警告する
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod event;
mod file;
mod ui;
mod error;

pub use app::App;
pub use ui::Render;

/// アプリケーション名
pub const APP_NAME: &str = "Resizer";
