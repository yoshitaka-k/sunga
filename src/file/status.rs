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

impl Status {
    /// リサイズステータスを文字列に変換
    /// * `return` - リサイズステータスを文字列に変換
    pub fn to_string(&self) -> String {
        match self {
            Status::Standby => lang::current().resize_status.standby.to_string(),
            Status::Resizing => lang::current().resize_status.resizing.to_string(),
            Status::Resized => lang::current().resize_status.resized.to_string(),
            Status::Skipped => lang::current().resize_status.skipped.to_string(),
            Status::Unchanged => lang::current().resize_status.unchanged.to_string(),
            Status::Canceled => lang::current().resize_status.canceled.to_string(),
            Status::Error(message) => format!("Error: {}", message),
        }
    }
}
