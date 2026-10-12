/// リサイズ状態
#[derive(Clone)]
pub enum Status {
    /// リサイズ未実行
    Standby,
    /// リサイズ中
    Resizing,
    /// リサイズ完了
    Resized,
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
            Status::Standby => "Standby".to_string(),
            Status::Resizing => "Resizing".to_string(),
            Status::Resized => "Resized".to_string(),
            Status::Canceled => "Canceled".to_string(),
            Status::Error(message) => format!("Error: {}", message),
        }
    }
}
