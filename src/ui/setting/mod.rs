mod view;
mod top;
mod general;
mod about;

pub(crate) use view::view;

// ウィンドウのID
const SETTING_WINDOW_ID: &str = "setting_window";

// ウィンドウのサイズ
const WINDOW_WIDTH: f32 = 420.0;
const WINDOW_HEIGHT: f32 = 240.0;

const TOP_PANEL_NAME: &str = "tab_panel";
