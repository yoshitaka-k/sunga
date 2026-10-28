use std::fmt;
use crate::ui::lang;

/// リサイズ状態
#[derive(Clone)]
pub enum Status {
    /// リサイズ未実行
    Standby,
    /// リサイズ中
    Resizing,
    /// リサイズ完了
    Resized,
    /// 同じパスはスキップ
    Skipped,
    /// 変更なし
    Unchanged,
    /// リサイズキャンセル
    Canceled,
    /// リサイズエラー（メッセージ）
    Error(String),
}

impl fmt::Display for Status {
    /// リサイズステータスを文字列に変換
    /// * `return` - リサイズステータスを文字列に変換
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = &lang::current().resize_status;
        match self {
            Status::Standby => write!(f, "{}", status.standby),
            Status::Resizing => write!(f, "{}", status.resizing),
            Status::Resized => write!(f, "{}", status.resized),
            Status::Skipped => write!(f, "{}", status.skipped),
            Status::Unchanged => write!(f, "{}", status.unchanged),
            Status::Canceled => write!(f, "{}", status.canceled),
            Status::Error(message) => write!(f, "Error: {}", message),
        }
    }
}
