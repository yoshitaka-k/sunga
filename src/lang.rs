mod generated {
    include!(concat!(env!("OUT_DIR"), "/lang_generated.rs"));
}

pub(crate) use generated::*;

/// 現在の言語を返す
/// * `return` - 現在の言語
pub fn current() -> &'static Labels {
    // TODO: 設定値を見て &EN か &JA を返す
    &EN
}
