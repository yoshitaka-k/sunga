use crate::{event, state, ui};
use crate::ui::assets::icon;
use crate::ui::main::{self, top, bottom, middle};
use crate::ui::{modal, setting};

/// 描画
/// * `ui` - UI
pub(crate) fn view(
    ui: &mut egui::Ui,
    state: &mut state::State,
    pending_actions: &mut Vec<event::EventAction>,
    status_color: &ui::StatusColor,
) {
    // 行の高さを取得
    let row_height = ui.text_style_height(&egui::TextStyle::Body)
        .max(icon::LIST_ICON_SIZE) + main::SEPARATOR_HEIGHT;

    // トップメニューバーを描画
    top::view(
        ui,
        state,
        pending_actions,
    );

    // 下部メニューバーを描画
    bottom::view(
        ui,
        state,
        pending_actions,
        status_color,
    );

    // 中央パネルを描画
    middle::view(
        ui,
        state,
        pending_actions,
        status_color,
        row_height,
    );

    // 設定ウィンドウを描画している場合は描画しない
    if state.setting_token.open {
        setting::view(
            ui,
            state,
            pending_actions,
        );
    }

    // エラーメッセージを描画
    if state.error_token.open {
        modal::error(ui.ctx(), &mut state.error_token);
    }
}

