use std::sync::{Arc, mpsc};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::file;

/// スキャントークン
#[derive(Clone)]
pub(crate) struct ScanToken {
    pub ctx: egui::Context,
    pub result_tx: mpsc::Sender<file::ScanResult>,
    pub running: Arc<AtomicBool>,
}

impl ScanToken {
    /// キャンセルされたかどうか
    /// * `return` - キャンセルされたかどうか
    pub fn is_canceled(&self) -> bool {
        // ジョブ実行中でない場合はキャンセルされたと判定
        !self.running.load(Ordering::Acquire)
    }
}
