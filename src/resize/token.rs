use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashSet;
use crate::error;

/// リサイズトークン
#[derive(Clone)]
pub struct Token {
    pub id: u64,
    pub running: Arc<AtomicBool>,
    pub canceled: Arc<Mutex<HashSet<u64>>>,
}

impl Token {
    /// キャンセルされたかどうか
    /// * `return` - キャンセルされたかどうか
    pub fn is_canceled(&self) -> error::Result<bool> {
        let canceled = self.canceled.lock().map_err(|_| {
            error::SungaError::lock_poisoned()
        })?;

        // リサイズ実行外またはキャンセルされた場合はキャンセルされたと判定
        Ok(!self.running.load(Ordering::Acquire) || canceled.contains(&self.id))
    }
}
